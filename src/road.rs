use bevy::render::mesh::{Indices, Mesh};
use bevy::math::{Vec3, vec3};
use bevy::render::pipeline::PrimitiveTopology;

pub type Shift = (f32, Vec3);
pub type SectionDescription = Vec<Shift>;

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


pub fn generate_sections() -> Vec<SectionDescription> {
    let x = 50f32;
    let y = 50f32;
    let width = 30f32;
    vec![
        vec![
             (width, vec3(x, 0.0, 0.0))
        ],
        vec![
             (width, vec3(x, 0.0, 0.0)),
        ],
        vec![
             (width, vec3(x, y, 0.0)),
             (width, vec3(x, 0.0, 0.0)),
             (width, vec3(x, -y, 0.0)),
             (width, vec3(000.0, -y, 0.0)),
             (width, vec3(000.0, -y, 0.0)),
        ],
        vec![
             (width, vec3(-x, -y, 0.0)),
             (width, vec3(-x, -y, 0.0)),
             (width, vec3(-x, -y, 0.0)),
             (width, vec3(-0.0, -y, 0.0)),
             (width, vec3(x, -0.0, 0.0)),
             (width, vec3(x, -0.0, 0.0)),
             (width, vec3(x, y, 0.0)),
             (width, vec3(x, y, 0.0)),
             (width, vec3(x, y, 0.0)),
             (width, vec3(x, y, 0.0)),
             (width, vec3(0.0, y, 0.0)),
             (width, vec3(0.0, y, 0.0)),
             (width, vec3(0.0, y, 0.0)),
             (width, vec3(0.0, y, 0.0)),
             (width, vec3(0.0, y, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, y, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, 0.0, 0.0)),
             (width, vec3(-x, -y, 0.0)),
             (width, vec3(0.0, -y, 0.0)),
             (width, vec3(0.0, -y, 0.0)),
             (width, vec3(x, -y, 0.0)),
             (width, vec3(x, 0.0, 0.0)),
        ],
    ]
}

pub fn generate_road(sections: &[SectionDescription]) -> Mesh {
    let mut trajectory = Vec::new();
    let mut current_pivot = Pivot {
        position: Vec3::ZERO,
        direction: Vec3::X,
        width: 30f32
    };
    for section in sections.iter() {
        let (new_pivot, section_trajectory) = build_section(current_pivot, section);
        trajectory.push(section_trajectory);
        current_pivot = new_pivot;
    }
    trajectory_to_mesh(trajectory)
}

pub fn sections_into_line(sections: &[SectionDescription]) -> Vec<Vec3> {
    let mut result = Vec::with_capacity(100);
    let mut position = Vec3::ZERO;
    result.push(position);
    for section in sections.iter() {
        for (_width, shift) in section.iter() {
            position += *shift;
            result.push(position);
        }
    }
    result
}

pub fn build_section(pivot: Pivot, description: &SectionDescription) -> (Pivot, Trajectory) {
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
    let mut current_normal = normal(pivot.direction);
    let mut current_direction = pivot.direction;
    let mut current_width = pivot.width;
    let mut trajectory_positions = Vec::new();
    for index in 0..(positions.len() - 1) {
        let new_width = *section_width.get(index).unwrap();
        let first = *positions.get(index).unwrap();
        let next = *positions.get(index + 1).unwrap();
        let new_direction = next - first;
        let new_normal = normal(new_direction);
        let first_middle = first + new_normal * new_width;
        let first_right = next + new_normal * new_width;
        let first_bottom = first - new_normal * new_width;
        let next_middle = first_right;
        let next_left = first_bottom;
        let next_bottom = next - new_normal * new_width;
        if new_normal != current_normal {
            trajectory_positions.push(first_bottom);
            trajectory_positions.push(first_middle);
            let first_left = first + current_normal * current_width;
            trajectory_positions.push(first_left);
            // temporary solution. I don't know how to process this case in any other way.
            trajectory_positions.push(first_bottom);
            trajectory_positions.push(first_middle);
            let first_left = first - current_normal * current_width;
            trajectory_positions.push(first_left);
        }
        trajectory_positions.push(first_middle);
        trajectory_positions.push(first_right);
        trajectory_positions.push(first_bottom);
        trajectory_positions.push(next_bottom);
        trajectory_positions.push(next_left);
        trajectory_positions.push(next_middle);
        current_width = new_width;
        current_normal = new_normal;
        current_direction = new_direction
    }
    let trajectory = Trajectory {
        positions: trajectory_positions.iter().map(|position| [position.x, position.y, position.z]).collect(),
        normals: (0..trajectory_positions.len()).map(|_| [1.0, 1.0, 1.0]).collect(),
        uvs: (0..trajectory_positions.len()).map(|_| [0.0, 0.0]).collect()
    };
    let current_position = *positions.last().unwrap();
    (
        Pivot {
            position: current_position,
            direction: current_direction,
            width: current_width
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

pub fn normal(vec: Vec3) -> Vec3 {
    // TODO: bug expected if direction is close to 0.
    vec.cross(Vec3::Z).normalize()
}
