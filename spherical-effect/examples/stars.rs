mod app;
use app::App;

use spherical_effect::*;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::{AdapterInfo, SwapChainFrame};
use winit::{dpi::*, event::*, event_loop::ControlFlow};

struct MyApp {
    scene: Scene,
    rotate_flag: bool,
    prev_cursor: Vector2,
}

const CODE: &'static str = "
const vec4 COSMIC = vec4(0, 25, 45, 255) / 255.0;
const vec4 STAR = vec4(141, 171, 197, 255) / 255.0;

// https://www.shadertoy.com/view/4djSRW
vec3 hash31(float p) {
   vec3 p3 = fract(vec3(p) * vec3(.1031, .1030, .0973));
   p3 += dot(p3, p3.yzx+33.33);
   return fract((p3.xxy+p3.yzz)*p3.zyx); 
}

vec4 sphericalColor(in vec3 dir) {
    float a = 0.0;
    for (float i = 1.; i <= 500.; i++) {
        vec3 p = normalize(2.0 * hash31(i) - 1.0);
        a += clamp(1.0 - distance(p, dir) / 0.005, 0.0, 1.0);
    }
    a = clamp(a, 0.0, 1.0);
    return mix(COSMIC, STAR, a);
}
";

impl App for MyApp {
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        let camera = Camera::default();
        let mut scene = Scene::new(
            device_handler.clone(),
            &SceneDescriptor {
                camera,
                ..Default::default()
            },
        );

        let bg = SphericalBackground::new(CODE);
        scene.add_object(&bg);

        // Return the application handler
        MyApp {
            scene,
            rotate_flag: false,
            prev_cursor: Vector2::zero(),
        }
    }
    // Called when the mouse button is pressed and released.
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) -> ControlFlow {
        match button {
            // Behavior when the left button is pressed or unpressed
            MouseButton::Left => {
                // pressed => start dragging, released => end dragging.
                self.rotate_flag = state == ElementState::Pressed;
            }
            _ => {}
        }
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }

    // Called when the cursor is moved
    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) -> ControlFlow {
        let position = Vector2::new(position.x, position.y);
        if self.rotate_flag {
            // get the mutable references of camera and light
            let desc = self.scene.descriptor_mut();
            let camera = &mut desc.camera;
            // get the delta of cursor move
            let dir2d = position - self.prev_cursor;
            // Do nothing if the delta is so small.
            if dir2d.so_small() {
                return Self::default_control_flow();
            }
            // axis of rotation
            let axis = (dir2d[1] * camera.matrix[0].truncate()
                + dir2d[0] * camera.matrix[1].truncate())
            .normalize();
            // angle of rotation. 0.01 times the pixel distance.
            let angle = dir2d.magnitude() * 0.01;
            // rotation matrix. The rotation angle is minus, as the camera is moved.
            let mat = Matrix4::from_axis_angle(axis, Rad(-angle));
            // move the camera and light.
            camera.matrix = mat * camera.matrix;
        }
        // assign the current cursor position to "previous cursor position"
        self.prev_cursor = position;
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) {
        // scene draws a picture to the window.
        self.scene.render_scene(&frame.output.view);
    }
}

// Run!
fn main() { MyApp::run() }
