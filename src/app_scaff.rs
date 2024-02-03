mod app_scaff;

use std::f32::consts::PI;
use kart_apple_gl::cgmath::{Angle, Deg, PerspectiveFov, Rad};
use kart_apple_gl::core::application::KartApple;
use kart_apple_gl::core::attribute::Attribute;
use kart_apple_gl::core::camera::Camera;
use kart_apple_gl::core::gl_var_type::GLvartype;
use kart_apple_gl::core::scaffold::AppScaffold;
use kart_apple_gl::{gl, glfw};
use kart_apple_gl::gl::types::GLuint;
use kart_apple_gl::glfw::{Action, Context, Key, Modifiers, MouseButton, Scancode};
use kart_apple_gl::util::program_utils::ProgramUtils;

pub struct PhysicsScaffold {
    cam: Option<Camera>,
    vao: GLuint,
    program: GLuint
}

impl PhysicsScaffold {
    pub fn new() -> PhysicsScaffold {
        PhysicsScaffold {
            cam: None,
            vao: GLuint::from(1u32),
            program: GLuint::from(1u32),
        }
    }
    fn cam(&mut self) -> &mut Camera {
        match &mut self.cam {
            Some(cam) => {

                return cam
            }
            _ => {
                panic!("cam has not been initialized")
            }
        }

    }
}

impl AppScaffold for PhysicsScaffold {
    unsafe fn on_init(&mut self, app: &mut KartApple) {

        gl::Enable(gl::DEPTH_TEST);
        self.program = ProgramUtils::create_program(

            &include_str!("../shaders/vert.glsl").to_string(),
            &include_str!("../shaders/frag.glsl").to_string()

        );
        let mut vao = GLuint::from(1u32);
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let verts = [
            -0.5f32, 0.5f32, 0.3f32,
            -0.5f32, -0.5f32, 0.3f32,
            0.5f32, -0.5f32, 0.3f32,
            0.5f32, 0.5f32, 0.3f32,

        ];
        let vert_colors = [
            0f32, 1f32, 0f32,
            1f32, 0f32, 0f32,
            0f32, 0f32, 1f32,
            0f32, 1f32, 1f32,

        ];
        Attribute::init(&vert_colors);
        Attribute::locate_attribute(self.program, "vert_color", GLvartype::Vec3);
        self.cam = Some(Camera::new(self.program, "model", "view", "proj"));
        self.cam().projection(
            PerspectiveFov {
                fovy: Rad(90f32 * (PI / 180f32)),
                aspect: (app.window.width as f32 / app.window.height as f32),
                near: (0.1f32),
                far: (50f32),
            }
        );
        self.cam().translate_view_z(-3.50f32);



        Attribute::init(&verts);
        Attribute::locate_attribute(self.program, "pos", GLvartype::Vec3);

    }

    unsafe fn on_loop(&mut self, app: &mut KartApple) {
        gl::ClearColor(0.0,0.0,0.0,1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        if glfw::ffi::glfwGetKey(app.window.glfw_win.window_ptr(), glfw::ffi::KEY_W) == glfw::ffi::PRESS {
            self.cam().translate_view_z(app.delta.value as f32 / 100000f32)
        }
        if glfw::ffi::glfwGetKey(app.window.glfw_win.window_ptr(), glfw::ffi::KEY_S) == glfw::ffi::PRESS {
            self.cam().translate_view_z(-1f32 * (app.delta.value as f32 / 100000f32))
        }
        if glfw::ffi::glfwGetKey(app.window.glfw_win.window_ptr(), glfw::ffi::KEY_A) == glfw::ffi::PRESS {
            self.cam().translate_view_x( 1f32 * (app.delta.value as f32 / 100000f32));

        }
        if glfw::ffi::glfwGetKey(app.window.glfw_win.window_ptr(), glfw::ffi::KEY_D) == glfw::ffi::PRESS {
            self.cam().rotate_view_y( -1f32 * (app.delta.value as f32) / 1000f32);
        }



        self.cam().rotate_view_x(app.delta.value as f32 / 100000f32);

        gl::UseProgram(self.program);
        // self.cam().rotate_model_x(1f32 * (app.delta.value as f32) / 100000f32 );
        self.cam().update();

        gl::DrawArrays(gl::QUADS, 0, 4);

    }

    unsafe fn on_key(&mut self, key: Key, scan_code: Scancode, action: Action, modifiers: Modifiers, app: &mut KartApple) {
    }

    unsafe fn on_mouse(&mut self, button: MouseButton, action: Action, modifiers: Modifiers, app: &mut KartApple) {
        todo!()
    }

    unsafe fn on_resize(&mut self, width: i32, height: i32, app: &mut KartApple) {
        gl::Viewport(0,0,width,height);
    }

    unsafe fn on_clean(&mut self, app: &mut KartApple) {
        gl::DeleteVertexArrays(1, &self.vao);
        gl::DeleteBuffers(1, &self.vao);
        gl::DeleteProgram(self.program);
    }
}