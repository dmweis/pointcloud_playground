
use kiss3d::window::Window;
use nalgebra::Point3;

use crate::PointCloud;


pub fn render_pointcloud(pointcloud: &PointCloud) {
    let mut window = Window::new("point_cloud_view");

    let mut points = Vec::new();

    for point in pointcloud.iter_points() {
        let pos = Point3::new(point.y, point.z, point.x);
        points.push(pos);
    }

    while window.render() {
        for point in points.iter() {
            let color = Point3::new(1.0, 0.0, 0.0);
            window.draw_point(&point, &color);
        }
    }
}
