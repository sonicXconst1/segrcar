use bevy::{
    app::{AppBuilder, Plugin, CoreStage},
    asset::{Assets, Handle},
    ecs::{
        system::{ResMut, Query, Commands},
        bundle::Bundle,
    },
    math::{Vec3, Vec4},
    prelude::{AddAsset, IntoSystem, ParallelSystemDescriptorCoercion},
    reflect::TypeUuid,
    render::{
        color::Color,
        entity::MeshBundle,
        mesh::{Indices, Mesh, VertexAttributeValues},
        pipeline::{PrimitiveTopology, PipelineDescriptor, RenderPipelines, RenderPipeline},
        render_graph::{
            base::node::MAIN_PASS,
            AssetRenderResourcesNode, 
            RenderGraph
        },
        renderer::RenderResources,
        shader::{Shader, ShaderStage, ShaderStages, ShaderDefs, asset_shader_defs_system}
    }
};

pub struct Line {
    pub start: Vec3,
    pub stop: Vec3
}

#[derive(Bundle)]
pub struct LineBundle {
    pub line: Line
}

impl LineBundle {
    pub fn new(start: Vec3, stop: Vec3) -> Self {
        LineBundle {
            line: Line { start, stop }
        }
    }
}

pub fn create_line(parts: Vec<[Vec3; 2]>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let positions: Vec<[f32; 3]> = parts.into_iter()
        .flatten()
        .map(|line| [line.x, line.y, 0f32])
        .collect();
    let indices = (0..positions.len()).map(|value| value as u32).collect();
    let normals = (0..positions.len()).map(|_| [1.0, 1.0, 1.0]).collect::<Vec<[f32; 3]>>();
    let uvs = (0..positions.len()).map(|_| [0.0, 0.0]).collect::<Vec<[f32; 2]>>();
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float3(positions.into()));
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<LineShader>()
            .add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::Last, draw_lines_with_mesh.system().label("draw_lines"))
            .add_system_to_stage(
                CoreStage::Last,
                asset_shader_defs_system::<LineShader>.system().before("draw_lines")
            );
    }
}

#[derive(RenderResources, Default, ShaderDefs, TypeUuid)]
#[uuid = "f093e7c5-634c-45f8-a2af-7fcd0245f259"]
pub struct LineShader {
    #[render_resources(buffer)]
    pub points: Vec<Vec4>,
    pub color: Vec4,
}

pub const MAX_LINES: usize = 228;
pub const MAX_POINTS: usize = MAX_LINES * 2;

fn create_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let positions = Vec::with_capacity(100);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float3(positions.into()));
    mesh
}

fn setup(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut materials: ResMut<Assets<LineShader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_descriptor = PipelineDescriptor::default_config(
        ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex, include_str!("../shaders/line.vert"))),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment, include_str!("../shaders/line.frag"))))
        }
    );

    render_graph.add_system_node(
        "line_shader",
        AssetRenderResourcesNode::<LineShader>::new(false)
    );
    render_graph.add_node_edge("line_shader", MAIN_PASS).unwrap();

    let pipeline_handler = pipelines.add(pipeline_descriptor);
    let pipes = RenderPipelines::from_pipelines(
        vec![RenderPipeline::new(pipeline_handler)]);

    let mesh = create_mesh();
    let shader = materials.add(LineShader {
        points: vec![Vec4::ZERO; MAX_POINTS],
        color: Color::RED.into()
    });

    commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: pipes,
            ..Default::default()
        })
        .insert(Test)
        .insert(shader);
}

struct Test;

fn draw_lines_with_mesh(
    mut meshes: Query<(&Handle<Mesh>, &Test)>,
    mut mesh_resources: ResMut<Assets<Mesh>>,
    lines: Query<&Line>
) {
    fn draw_lines_on_mesh(
        positions: &mut Vec<[f32; 3]>,
        line_index: usize,
        line: &Line) {
        let line_index = line_index * 2;
        let next_line_index = line_index + 1;
        if next_line_index >= positions.len() {
            positions.push(line.start.into());
            positions.push(line.stop.into());
        } else {
            positions[line_index] = line.start.into();
            positions[next_line_index] = line.stop.into();
        }
    }

    for (mesh_handle, _shader) in meshes.iter_mut() {
        let mesh = mesh_resources.get_mut(mesh_handle)
            .expect("Invalid mesh handle");
        let positions = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh without positions");
        match positions {
            VertexAttributeValues::Float3(positions) => {
                for (index, line) in lines.iter().enumerate() {
                    draw_lines_on_mesh(positions, index, line)
                }
                //println!("Positions: {:?}", positions);
            },
            other => panic!("Invalid type of positions {:#?}", other)
        }
    }
}

#[allow(dead_code)]
fn draw_lines(
    mut assets: ResMut<Assets<LineShader>>,
    shaders: Query<&Handle<LineShader>>,
    lines: Query<&Line>
) {
    for shader in shaders.iter() {
        if let Some(shader) = assets.get_mut(shader) {
            for (index, line) in lines.iter().enumerate() {
                shader.points[index * 2] = line.start.extend(1f32);
                shader.points[index * 2 + 1] = line.stop.extend(1f32);
            }
        }
    }
}
