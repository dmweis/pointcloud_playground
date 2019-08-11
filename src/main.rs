use pointcloud_playground::Vector3;
use pointcloud_playground::PointCloud;
mod point_cloud_view;

fn main() {
    let vec = Vector3::new(3.0, 3.0, 3.0);
    println!("vector {:?}", vec);

    let cloud = PointCloud::load(String::from("test_output.csv")).unwrap();

    println!("Points in file");

    let mut point_sum = Vector3::new(0.0, 0.0, 0.0);

    for point in cloud.iter_points() {
        // println!("vector {:?}", point);
        point_sum = *point + point_sum;
    }

    println!("Point sum {:?}", point_sum);
    println!("Bounds {:?}", cloud.boundaries());

    point_cloud_view::render_pointcloud(&cloud);

}
