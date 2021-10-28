use bevy::render::mesh::{Indices, Mesh};
use bevy::math::{Vec3, vec3};
use bevy::render::pipeline::PrimitiveTopology;

pub type Shift = (f32, Vec3);
pub type SectionDescription = Vec<Shift>;

pub struct Trajectory {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>
}


pub enum Section {
    Straight(SectionDescription),
    Turn(SectionDescription),
}

pub fn generate_sections() -> Vec<Section> {
    use Section::*;
    vec![
        Straight(vec![
             (10f32, vec3(100.0, 0.0, 0.0)),
        ]),
        Straight(vec![
             (30f32, vec3(100.0, 100.0, 0.0)),
        ]),
        Straight(vec![
             (50f32, vec3(0.0, 120.0, 0.0)),
        ]),
    ]
}

pub fn generate_road(sections: &[Section]) -> Mesh {
    use Section::*;
    let mut trajectory = Vec::new();
    let mut current_position = Vec3::ZERO;
    let mut width = 10f32;
    for section in sections.iter() {
        let (new_width, new_position, section_trajectory) = match section {
            Straight(trajectory) => build_straight(current_position, width, trajectory),
            Turn(trajectory) => build_turn(current_position, width, trajectory)
        };
        trajectory.push(section_trajectory);
        current_position = new_position;
        width = new_width;
    }
    trajectory_to_mesh(trajectory)
}

pub fn build_straight(start: Vec3, width: f32, description: &SectionDescription) -> (f32, Vec3, Trajectory) {
    let mut positions = Vec::new();
    let mut section_width = Vec::new();
    let mut current_position = start;
    positions.push(start);
    section_width.push(width);
    for (width, point) in description.iter() {
        current_position += *point;
        positions.push(current_position);
        section_width.push(*width);
    }
    let mut trajectory_positions = Vec::new();
    for index in 0..(positions.len() - 1) {
        let width = *section_width.get(index).unwrap();
        let first = *positions.get(index).unwrap();
        let next = *positions.get(index + 1).unwrap();
        let direction = next - first;
        // TODO: bug expected if direction is close to 0.
        let perpendicular = vec3(direction.y, -direction.x, 0f32).normalize();
        let first_middle = first + perpendicular * width;
        let first_right = next + perpendicular * width;
        let first_bottom = first - perpendicular * width;
        let next_middle = first_right;
        let next_left = first_bottom;
        let next_bottom = next - perpendicular * width;
        trajectory_positions.push(first_middle);
        trajectory_positions.push(first_right);
        trajectory_positions.push(first_bottom);
        trajectory_positions.push(next_bottom);
        trajectory_positions.push(next_left);
        trajectory_positions.push(next_middle);
    }
    println!("Trajectory Positions: {:#?}", trajectory_positions);
    let trajectory = Trajectory {
        positions: trajectory_positions.iter().map(|position| [position.x, position.y, position.z]).collect(),
        indices: (0..trajectory_positions.len()).map(|index| index as u32).collect(),
        normals: (0..trajectory_positions.len()).map(|_| [1.0, 1.0, 1.0]).collect(),
        uvs: (0..trajectory_positions.len()).map(|_| [0.0, 0.0]).collect()
    };
    let current_position = *positions.last().unwrap();
    let width = *section_width.last().unwrap();
    (width, current_position, trajectory)
}

pub fn build_turn(start: Vec3, width: f32, description: &SectionDescription) -> (f32, Vec3, Trajectory) {
    unimplemented!()
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
