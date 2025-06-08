use avian2d::parry::shape::TriMesh;
use bevy::prelude::*;
use avian2d::prelude::*;
use std::path::Path;
use avian2d::parry::na::Point2;
use roxmltree::{Document, Node};
use crate::in_game::balls::level_ball::LevelBall;

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

fn load_level(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    level_entities: Query<Entity, Or<(With<LevelCollider>, With<LevelBall>)>>,
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

    // Process object layers
    for object_group in doc.descendants().filter(|n| n.has_tag_name("objectgroup")) {
        match object_group.attribute("name") {
            Some("static") => process_static_geometry(&mut commands, object_group),
            Some("balls") => process_balls(&mut commands, object_group),
            _ => continue,
        }
    }
}

fn process_static_geometry(commands: &mut Commands, object_group: Node) {
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
                let points = parse_polygon_points(points_str, x, y);
                spawn_collision_body(commands, points);
            }
        }
    }
}

fn process_balls(commands: &mut Commands, object_group: Node) {
    for object in object_group.children().filter(|n| n.has_tag_name("object")) {
        // Get ball position
        let x = object.attribute("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let y = object.attribute("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        // Invert Y coordinate
        let y = -y;
        
        // Spawn a level ball at this position
        commands.spawn((
            LevelBall {
                static_body: true
            },
            Transform::from_xyz(x, y, 0.0),
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