use bevy::render::mesh::{Indices, Mesh};
use bevy::math::{Vec3, vec3};
use bevy::render::pipeline::PrimitiveTopology;

pub type Shift = (f32, Vec3);
pub type Speed = Vec3;
pub type SectionDescription = Vec<Shift>;
pub type TurnDescription = Vec<(Shift, Speed)>;

pub struct Trajectory {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

pub struct Pivot {
    position: Vec3,
    direction: Vec3,
    width: f32
}

pub enum Section {
    Straight(SectionDescription),
    Turn(TurnDescription),
}

pub fn generate_sections() -> Vec<Section> {
    use Section::*;
    vec![
        Straight(vec![
             (25f32, vec3(100.0, 0.0, 0.0))
        ]),
        Turn(vec![
             ((25f32, vec3(100.0, 100.0, 0.0)), vec3(20.0, 30.0, 0.0))
        ]),
        Straight(vec![
             (25f32, vec3(100.0, 0.0, 0.0))
        ]),
        Turn(vec![
             ((25f32, vec3(100.0, -100.0, 0.0)), vec3(20.0, -30.0, 0.0))
        ]),
    ]
}

pub fn generate_road(sections: &[Section]) -> Mesh {
    use Section::*;
    let mut trajectory = Vec::new();
    let mut current_pivot = Pivot {
        position: Vec3::ZERO,
        direction: Vec3::X,
        width: 25f32
    };
    for section in sections.iter() {
        let (new_pivot, section_trajectory) = match section {
            Straight(trajectory) => build_straight(current_pivot, trajectory),
            Turn(turn) => build_turn(current_pivot, turn)
        };
        trajectory.push(section_trajectory);
        current_pivot = new_pivot;
    }
    trajectory_to_mesh(trajectory)
}

pub fn build_straight(pivot: Pivot, description: &SectionDescription) -> (Pivot, Trajectory) {
    let mut positions = Vec::new();
    let mut section_width = Vec::new();
    let mut current_position = pivot.position;
    positions.push(pivot.position);
    section_width.push(pivot.width);
    for (width, point) in description.iter() {
        current_position += *point;
        positions.push(current_position);
        section_width.push(*width);
    }
    let current_perpendicular = perpendicular(pivot.direction);
    let mut trajectory_positions = Vec::new();
    for index in 0..(positions.len() - 1) {
        let mut width = *section_width.get(index).unwrap();
        let first = *positions.get(index).unwrap();
        let next = *positions.get(index + 1).unwrap();
        let new_perpendicular = perpendicular(next - first);
        width /= new_perpendicular.angle_between(current_perpendicular).cos();
        let first_middle = first + current_perpendicular * width;
        let first_right = next + current_perpendicular * width;
        let first_bottom = first - current_perpendicular * width;
        let next_middle = first_right;
        let next_left = first_bottom;
        let next_bottom = next - current_perpendicular * width;
        trajectory_positions.push(first_middle);
        trajectory_positions.push(first_right);
        trajectory_positions.push(first_bottom);
        trajectory_positions.push(next_bottom);
        trajectory_positions.push(next_left);
        trajectory_positions.push(next_middle);
    }
    let trajectory = Trajectory {
        positions: trajectory_positions.iter().map(|position| [position.x, position.y, position.z]).collect(),
        normals: (0..trajectory_positions.len()).map(|_| [1.0, 1.0, 1.0]).collect(),
        uvs: (0..trajectory_positions.len()).map(|_| [0.0, 0.0]).collect()
    };
    let current_position = *positions.last().unwrap();
    let width = *section_width.last().unwrap();
    (
        Pivot {
            position: current_position,
            direction: pivot.direction,
            width
        },
        trajectory
    )
}

pub fn build_turn(pivot: Pivot, description: &TurnDescription) -> (Pivot, Trajectory) {
    let mut positions = Vec::new();
    let mut section_width = Vec::new();
    let mut current_position = pivot.position;
    positions.push(pivot.position);
    section_width.push(pivot.width);
    let steps_count = 3;
    for ((width, point), speed) in description.iter() {
        let final_position = current_position + *point;
        for _step in 0..steps_count {
            current_position += *speed;
            current_position = current_position.min(final_position);
            positions.push(current_position);
            section_width.push(*width);
        }
        if current_position != final_position {
            positions.push(final_position);
            section_width.push(*width);
        }
    }
    let mut current_perpendicular = perpendicular(pivot.direction);
    let mut current_direction = pivot.direction;
    let mut trajectory_positions = Vec::new();
    for index in 0..(positions.len() - 1) {
        let width = *section_width.get(index).unwrap();
        let first = *positions.get(index).unwrap();
        let next = *positions.get(index + 1).unwrap();
        let new_perpendicular = perpendicular(next - first);
        //width /= new_perpendicular.angle_between(current_perpendicular).cos();
        let first_middle = first + current_perpendicular * width;
        let first_right = next + new_perpendicular * width;
        let first_bottom = first - current_perpendicular * width;
        let next_middle = first_right;
        let next_left = first_bottom;
        let next_bottom = next - new_perpendicular * width;
        trajectory_positions.push(first_middle);
        trajectory_positions.push(first_right);
        trajectory_positions.push(first_bottom);
        trajectory_positions.push(next_bottom);
        trajectory_positions.push(next_left);
        trajectory_positions.push(next_middle);
        current_perpendicular = new_perpendicular;
        current_direction = next - first;
    }
    let trajectory = Trajectory {
        positions: trajectory_positions.iter().map(|position| [position.x, position.y, position.z]).collect(),
        normals: (0..trajectory_positions.len()).map(|_| [1.0, 1.0, 1.0]).collect(),
        uvs: (0..trajectory_positions.len()).map(|_| [0.0, 0.0]).collect()
    };
    let current_position = *positions.last().unwrap();
    let width = *section_width.last().unwrap();
    (
        Pivot {
            position: current_position,
            direction: current_direction,
            width
        },
        trajectory
    )
}

pub fn trajectory_to_mesh(descriptions: Vec<Trajectory>) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for description in descriptions.into_iter() {
        positions.extend(description.positions);
        normals.extend(description.normals);
        uvs.extend(description.uvs);
    }
    let indices = (0..positions.len()).map(|value| value as u32).collect();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh
}

fn perpendicular(vec: Vec3) -> Vec3 {
    // TODO: bug expected if direction is close to 0.
    vec3(vec.y, -vec.x, 0f32).normalize()
}
