//! Floor plan and indoor navigation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A point on a floor plan.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A room on a floor plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub floor: i32,
    pub corners: Vec<Point2D>,
    pub tags: Vec<String>,
}

/// A navigation waypoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: String,
    pub position: Point2D,
    pub floor: i32,
    pub name: String,
    pub connections: Vec<String>, // connected waypoint IDs
}

/// Floor plan data model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorPlan {
    pub id: String,
    pub name: String,
    pub floor: i32,
    pub width_m: f64,
    pub height_m: f64,
    pub rooms: Vec<Room>,
    pub waypoints: Vec<Waypoint>,
    pub beacons: Vec<BeaconPlacement>,
}

/// A beacon placed on the floor plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconPlacement {
    pub beacon_id: String,
    pub position: Point2D,
    pub beacon_type: String, // "wifi", "ble", "uwb"
}

/// Navigation graph for pathfinding.
pub struct NavigationGraph {
    waypoints: HashMap<String, Waypoint>,
}

impl NavigationGraph {
    pub fn from_floor_plan(plan: &FloorPlan) -> Self {
        let waypoints = plan
            .waypoints
            .iter()
            .map(|w| (w.id.clone(), w.clone()))
            .collect();
        Self { waypoints }
    }

    /// Find shortest path using Dijkstra's algorithm.
    pub fn find_path(&self, from_id: &str, to_id: &str) -> Option<Vec<String>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let _from = self.waypoints.get(from_id)?;
        let _to = self.waypoints.get(to_id)?;

        let mut dist: HashMap<String, f64> = HashMap::new();
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(from_id.to_string(), 0.0);
        heap.push(Reverse((
            ordered_float::OrderedFloat(0.0),
            from_id.to_string(),
        )));

        while let Some(Reverse((ordered_float::OrderedFloat(d), u))) = heap.pop() {
            if u == to_id {
                // Reconstruct path
                let mut path = vec![to_id.to_string()];
                let mut current = to_id.to_string();
                while let Some(p) = prev.get(&current) {
                    path.push(p.clone());
                    current = p.clone();
                }
                path.reverse();
                return Some(path);
            }

            if d > *dist.get(&u).unwrap_or(&f64::MAX) {
                continue;
            }

            if let Some(wp) = self.waypoints.get(&u) {
                for neighbor_id in &wp.connections {
                    if let Some(neighbor) = self.waypoints.get(neighbor_id) {
                        let dx = wp.position.x - neighbor.position.x;
                        let dy = wp.position.y - neighbor.position.y;
                        let edge_cost = (dx * dx + dy * dy).sqrt();
                        let new_dist = d + edge_cost;

                        if new_dist < *dist.get(neighbor_id).unwrap_or(&f64::MAX) {
                            dist.insert(neighbor_id.clone(), new_dist);
                            prev.insert(neighbor_id.clone(), u.clone());
                            heap.push(Reverse((
                                ordered_float::OrderedFloat(new_dist),
                                neighbor_id.clone(),
                            )));
                        }
                    }
                }
            }
        }
        None
    }

    /// Get total path distance in meters.
    pub fn path_distance(&self, path: &[String]) -> f64 {
        let mut total = 0.0;
        for i in 1..path.len() {
            if let (Some(a), Some(b)) = (
                self.waypoints.get(&path[i - 1]),
                self.waypoints.get(&path[i]),
            ) {
                let dx = a.position.x - b.position.x;
                let dy = a.position.y - b.position.y;
                total += (dx * dx + dy * dy).sqrt();
            }
        }
        total
    }

    /// Check which room a point is in (point-in-polygon).
    pub fn locate_room(plan: &FloorPlan, point: Point2D) -> Option<String> {
        for room in &plan.rooms {
            if point_in_polygon(&room.corners, point) {
                return Some(room.id.clone());
            }
        }
        None
    }
}

/// Ray-casting point-in-polygon test.
fn point_in_polygon(polygon: &[Point2D], point: Point2D) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;
    for i in 0..n {
        let pi = &polygon[i];
        let pj = &polygon[j];
        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_plan() -> FloorPlan {
        FloorPlan {
            id: "floor1".into(),
            name: "Ground Floor".into(),
            floor: 0,
            width_m: 50.0,
            height_m: 30.0,
            rooms: vec![Room {
                id: "lobby".into(),
                name: "Lobby".into(),
                floor: 0,
                corners: vec![
                    Point2D { x: 0.0, y: 0.0 },
                    Point2D { x: 20.0, y: 0.0 },
                    Point2D { x: 20.0, y: 15.0 },
                    Point2D { x: 0.0, y: 15.0 },
                ],
                tags: vec!["entrance".into()],
            }],
            waypoints: vec![
                Waypoint {
                    id: "a".into(),
                    position: Point2D { x: 0.0, y: 0.0 },
                    floor: 0,
                    name: "Start".into(),
                    connections: vec!["b".into()],
                },
                Waypoint {
                    id: "b".into(),
                    position: Point2D { x: 10.0, y: 0.0 },
                    floor: 0,
                    name: "Mid".into(),
                    connections: vec!["a".into(), "c".into()],
                },
                Waypoint {
                    id: "c".into(),
                    position: Point2D { x: 10.0, y: 10.0 },
                    floor: 0,
                    name: "End".into(),
                    connections: vec!["b".into()],
                },
            ],
            beacons: vec![],
        }
    }

    #[test]
    fn test_pathfinding() {
        let plan = make_test_plan();
        let graph = NavigationGraph::from_floor_plan(&plan);
        let path = graph.find_path("a", "c").unwrap();
        assert_eq!(path, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_path_distance() {
        let plan = make_test_plan();
        let graph = NavigationGraph::from_floor_plan(&plan);
        let dist = graph.path_distance(&["a".into(), "b".into(), "c".into()]);
        assert!((dist - 20.0).abs() < 0.01); // 10 + 10
    }

    #[test]
    fn test_point_in_room() {
        let plan = make_test_plan();
        let room = NavigationGraph::locate_room(&plan, Point2D { x: 5.0, y: 5.0 });
        assert_eq!(room, Some("lobby".into()));

        let outside = NavigationGraph::locate_room(&plan, Point2D { x: 25.0, y: 25.0 });
        assert_eq!(outside, None);
    }
}
