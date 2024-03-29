#![allow(dead_code)]
use cgmath::*;
use std::f32::consts::PI;
use winit::window::Window;

pub fn create_transforms(
    translation: [f32; 3],
    rotation: [f32; 3],
    scaling: [f32; 3],
) -> Matrix4<f32> {
    let trans_mat =
        Matrix4::from_translation(Vector3::new(translation[0], translation[1], translation[2]));
    let rotate_mat_x = Matrix4::from_angle_x(Rad(rotation[0]));
    let rotate_mat_y = Matrix4::from_angle_x(Rad(rotation[1]));
    let rotate_mat_z = Matrix4::from_angle_x(Rad(rotation[2]));
    let scale_mat = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

    let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat;
    model_mat
}

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 0.5, 0.0, //
    0.0, 0.0, 0.5, 1.0, //
);

pub fn create_view(
    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
) -> Matrix4<f32> {
    Matrix4::look_at_rh(camera_position, look_direction, up_direction)
}

pub fn create_projection(aspect: f32, is_perspective: bool) -> Matrix4<f32> {
    let project_mat: Matrix4<f32>;
    if is_perspective {
        project_mat = OPENGL_TO_WGPU_MATRIX * perspective(Rad(2.0 * PI / 5.0), aspect, 0.1, 100.0);
    } else {
        project_mat = OPENGL_TO_WGPU_MATRIX * ortho(-4.0, 4.0, -3.0, 3.0, -1.0, 6.0);
    }
    project_mat
}

pub fn create_view_projection(
    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
    aspect: f32,
    is_perspective: bool,
) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
    let view_mat = create_view(camera_position, look_direction, up_direction);
    let project_mat = create_projection(aspect, is_perspective);
    let view_project_mat = project_mat * view_mat;
    (view_mat, project_mat, view_project_mat)
}

pub struct InitWgpu {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl InitWgpu {
    pub async fn init_wgpu(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe {
            instance.create_surface(window)
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface)
            })
            .await
            .unwrap();

        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                }, None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }

    }
}
