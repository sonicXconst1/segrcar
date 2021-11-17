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

#[derive(Debug)]
pub struct Point(pub Vec3, pub Vec3);

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Point>,
    pub color: Color
}

#[derive(Bundle)]
pub struct LineBundle {
    pub line: Line
}

impl LineBundle {
    pub fn from_points(points: Vec<Point>, color: Color) -> Self {
        LineBundle {
            line: Line { points, color }
        }
    }

    pub fn from_line(line: Vec<Vec3>, color: Color) -> Self {
        let mut points = Vec::with_capacity(line.len());
        let mut current = *line.first().expect("Empty Line");
        for point in line.into_iter() {
            points.push(Point(current, point));
            current = point;
        }
        LineBundle {
            line: Line { points, color }
        }
    }

    pub fn from_mesh(mesh: &Mesh) -> Self {
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh without positions");
        let positions = match positions {
            VertexAttributeValues::Float3(positions) => positions,
            other => panic!("Invalid type of positions {:#?}", other)
        };
        let indices = match mesh.indices().expect("Indices are available") {
            Indices::U16(_) => panic!("u16 is unsupported type"),
            Indices::U32(indices) => indices
        };
        let mut points: Vec<Point> = Vec::with_capacity(positions.len() * 3);
        for index in indices.chunks(3) {
            let position_0 = *positions.get(index[0] as usize).expect("Invalid index");
            let position_1 = *positions.get(index[1] as usize).expect("Invalid index");
            let position_2 = *positions.get(index[2] as usize).expect("Invalid index");
            points.push(Point(position_0.into(), position_1.into()));
            points.push(Point(position_1.into(), position_2.into()));
            points.push(Point(position_2.into(), position_0.into()));
        }
        Self::from_points(points, Color::GREEN)
    }
}

pub fn line_to_points(line: &[Vec3]) -> Vec<Point> {
    let mut points = Vec::with_capacity(line.len());
    let mut current = *line.first().expect("Empty Line");
    for point in line.iter() {
        points.push(Point(current, *point));
        current = *point;
    }
    points
}

pub fn line_to_normals(line: &[Point]) -> Vec<Point> {
    fn normal(vec: Vec3) -> Vec3 {
        // TODO: bug expected if direction is close to 0.
        vec.cross(Vec3::Z).normalize()
    }
    line.iter()
        .map(|Point(start, stop)| {
            [
                Point(*stop, normal(*stop - *start) * 40f32 + *stop),
                Point(*start, normal(*stop - *start) * 40f32 + *start),
            ]
        })
        .flatten()
        .collect()
}

pub fn create_line(parts: Vec<[Vec3; 2]>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
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
pub struct LineShader{
    #[render_resources(buffer)]
    colors: Vec<Vec4>
}

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
    let shader = materials.add(LineShader { colors: Vec::new()});

    commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: pipes,
            ..Default::default()
        })
        .insert(shader);
}

fn draw_lines_with_mesh(
    mut meshes: Query<(&Handle<Mesh>, &Handle<LineShader>)>,
    mut mesh_resources: ResMut<Assets<Mesh>>,
    mut shader_resource: ResMut<Assets<LineShader>>,
    lines: Query<&Line>
) {
    fn draw_point(
        positions: &mut Vec<[f32; 3]>,
        colors: &mut Vec<Vec4>,
        point: Vec3,
        color: Color,
        index: usize,
    ) {
        if index >= positions.len() {
            positions.push(point.into());
            colors.push(color.into());
        } else {
            positions[index] = point.into();
            colors[index] = color.into();
        }
    }

    fn draw_lines_on_mesh(
        positions: &mut Vec<[f32; 3]>,
        colors: &mut Vec<Vec4>,
        point_counter: usize,
        line: &Line) {
        for (index, Point(start, stop)) in line.points.iter().enumerate() {
            let point_index = point_counter + index * 2;
            let next_point_index = point_index + 1;
            draw_point(positions, colors, *start, line.color, point_index);
            draw_point(positions, colors, *stop, line.color, next_point_index)
        }
    }

    for (mesh_handle, shader_handle) in meshes.iter_mut() {
        let shader = shader_resource.get_mut(shader_handle)
            .expect("Invalid shader handle");
        let mesh = mesh_resources.get_mut(mesh_handle)
            .expect("Invalid mesh handle");
        let positions = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh without positions");
        let mut point_counter = 0;
        match positions {
            VertexAttributeValues::Float3(positions) => {
                for line in lines.iter() {
                    //println!("Line {:#?}", line);
                    draw_lines_on_mesh(
                        positions,
                        &mut shader.colors,
                        point_counter,
                        line);
                    point_counter += line.points.len() * 2;
                }
                //println!("Positions: {:?}", positions);
            },
            other => panic!("Invalid type of positions {:#?}", other)
        }
    }
}
