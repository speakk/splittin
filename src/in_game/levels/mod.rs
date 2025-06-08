use avian2d::parry::shape::TriMesh;
use bevy::prelude::*;
use avian2d::prelude::*;
use std::path::Path;
use avian2d::parry::na::Point2;
use roxmltree::{Document, Node};
use crate::in_game::balls::level_ball::LevelBall;
use crate::in_game::player::Player;

pub struct LevelLoadingPlugin;

impl Plugin for LevelLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_level);
    }
}

#[derive(Component)]
pub struct LevelCollider;

#[derive(Resource, Clone)]
pub struct CurrentLevel {
    pub path: String,
}

// Helper struct to track level bounds
struct LevelBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl LevelBounds {
    fn new() -> Self {
        Self {
            min_x: f32::MAX,
            max_x: f32::MIN,
            min_y: f32::MAX,
            max_y: f32::MIN,
        }
    }

    fn update(&mut self, x: f32, y: f32) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn center(&self) -> Vec2 {
        Vec2::new(
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }
}

fn load_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    level_entities: Query<Entity, Or<(With<LevelCollider>, With<LevelBall>, With<Player>)>>,
) {
    // Only run if CurrentLevel has changed
    if !current_level.is_changed() {
        return;
    }

    let path = Path::new(&current_level.path);
    if !path.exists() {
        error!("Level file not found: {}", current_level.path);
        return;
    }

    // Read TMX file content
    let tmx_content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read TMX file: {}", e);
            return;
        }
    };

    // Parse TMX content
    let doc = match Document::parse(&tmx_content) {
        Ok(doc) => doc,
        Err(e) => {
            error!("Failed to parse TMX file: {}", e);
            return;
        }
    };

    // Clean up existing level entities
    for entity in level_entities.iter() {
        commands.entity(entity).despawn();
    }

    // Calculate level bounds first
    let mut bounds = LevelBounds::new();
    for object_group in doc.descendants().filter(|n| n.has_tag_name("objectgroup")) {
        calculate_layer_bounds(&mut bounds, object_group);
    }

    // Calculate the offset to center the level
    let center_offset = bounds.center();

    // Process object layers with centering offset
    for object_group in doc.descendants().filter(|n| n.has_tag_name("objectgroup")) {
        match object_group.attribute("name") {
            Some("static") => process_static_geometry(&mut commands, object_group, center_offset),
            Some("balls") => process_balls(&mut commands, object_group, center_offset),
            Some("player") => process_players(&mut commands, object_group, center_offset),
            _ => continue,
        }
    }
}

fn calculate_layer_bounds(bounds: &mut LevelBounds, object_group: Node) {
    for object in object_group.children().filter(|n| n.has_tag_name("object")) {
        let x = object.attribute("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let y = object.attribute("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let y = -y; // Invert Y coordinate

        bounds.update(x, y);

        // For polygon objects, also check their points
        if let Some(polygon) = object.children().find(|n| n.has_tag_name("polygon")) {
            if let Some(points_str) = polygon.attribute("points") {
                for point_str in points_str.split_whitespace() {
                    let mut coords = point_str.split(',');
                    if let (Some(dx), Some(dy)) = (
                        coords.next().and_then(|s| s.parse::<f32>().ok()),
                        coords.next().and_then(|s| s.parse::<f32>().ok()),
                    ) {
                        bounds.update(x + dx, y - dy);
                    }
                }
            }
        }
    }
}

fn process_static_geometry(commands: &mut Commands, object_group: Node, center_offset: Vec2) {
    for object in object_group.children().filter(|n| n.has_tag_name("object")) {
        // Find polygon child element
        if let Some(polygon) = object.children().find(|n| n.has_tag_name("polygon")) {
            // Get object position
            let x = object.attribute("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let y = object.attribute("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            // Invert Y coordinate
            let y = -y;
            
            // Parse polygon points
            if let Some(points_str) = polygon.attribute("points") {
                let points = parse_polygon_points(points_str, x - center_offset.x, y - center_offset.y);
                spawn_collision_body(commands, points);
            }
        }
    }
}

fn process_balls(commands: &mut Commands, object_group: Node, center_offset: Vec2) {
    for object in object_group.children().filter(|n| n.has_tag_name("object")) {
        // Get ball position
        let x = object.attribute("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let y = object.attribute("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        // Invert Y coordinate and apply centering offset
        let y = -y;
        
        // Spawn a level ball at this position
        commands.spawn((
            LevelBall {
                static_body: true
            },
            Transform::from_xyz(x - center_offset.x, y - center_offset.y, 0.0),
        ));
    }
}

fn process_players(commands: &mut Commands, object_group: Node, center_offset: Vec2) {
    for object in object_group.children().filter(|n| n.has_tag_name("object")) {
        // Get player position
        let x = object.attribute("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let y = object.attribute("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        // Invert Y coordinate and apply centering offset
        let y = -y;
        
        // Spawn a player at this position
        commands.spawn((
            Player,
            Transform::from_xyz(x - center_offset.x, y - center_offset.y, 0.0),
        ));
    }
}

fn parse_polygon_points(points_str: &str, base_x: f32, base_y: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = points_str
        .split_whitespace()
        .filter_map(|point_str| {
            let mut coords = point_str.split(',');
            let x = coords.next()?.parse::<f32>().ok()?;
            let y = coords.next()?.parse::<f32>().ok()?;
            // Invert Y coordinate for relative points
            Some(Vec2::new(base_x + x, base_y - y))
        })
        .collect();
    
    // Reverse the winding order to maintain correct orientation after Y-flip
    points.reverse();
    points
}

fn spawn_collision_body(commands: &mut Commands, points: Vec<Vec2>) {
    if points.len() < 3 {
        return; // Need at least 3 points for a polygon
    }

    // Convert Vec2 points to Point2<f32> format for TriMesh::from_polygon
    let points_array: Vec<Point2<f32>> = points.iter()
        .map(|p| Point2::new(p.x, p.y))
        .collect();

    // Create the trimesh from the polygon points
    if let Some(trimesh) = TriMesh::from_polygon(points_array) {
        let vertices: Vec<Vec2> = trimesh.vertices()
            .iter()
            .map(|p| Vec2::new(p.x, p.y))
            .collect();
        
        let indices: Vec<[u32; 3]> = trimesh.indices().to_vec();

        commands.spawn((
            RigidBody::Static,
            Collider::trimesh(vertices, indices),
            LevelCollider,
        ));
    } else {
        warn!("Failed to create trimesh from polygon points");
    }
}

fn triangulate_polygon(points: &[Vec2]) -> Vec<[u32; 3]> {
    // Simple triangulation for convex polygons
    // For more complex polygons, you might want to use a proper triangulation library
    let mut indices = Vec::new();
    for i in 1..(points.len() - 1) {
        indices.push([0, i as u32, (i + 1) as u32]);
    }
    indices
} 