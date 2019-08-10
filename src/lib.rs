use std::ops::{Add, Sub};
use std::error::Error;
use std::fs;
use std::slice::Iter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {x, y, z}
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

pub struct PointCloud {
    points: Vec<Vector3>
}

impl PointCloud {
    fn new() -> PointCloud {
        let points = Vec::new();
        PointCloud {  points }
    }

    pub fn load(filepath: String) -> Result<PointCloud, Box<dyn Error>> {
        let content = fs::read_to_string(filepath)?;

        let mut pointcloud = PointCloud::new();

        for line in content.lines().skip(1) {
            let elements: Vec<f32> = line
                            .trim()
                            .split(", ")
                            .map(|num| num.parse::<f32>().unwrap())
                            .collect();
            let x = elements[0];
            let y = elements[1];
            let z = elements[2];
            let point = Vector3::new(x, y, z);
            pointcloud.points.push(point);
        }

        Ok(pointcloud)
    } 

    pub fn iter_points(&self) -> Iter<Vector3> {
        self.points.iter()
    }

    pub fn add_point(&mut self, point: Vector3) {
        self.points.push(point);
    }

    pub fn is_empty(&self) -> bool {
        self.points.len() == 0
    }

    pub fn sub_box(self, min: Vector3, max: Vector3) -> PointCloud {
        let mut pointcloud = PointCloud::new();

        // ugly..... figure out a nicer way
        for point in self.points {
            if point.x >= min.x &&
                point.y >= min.y &&
                point.z >= min.z && 
                point.x <= max.x &&
                point.y <= max.y &&
                point.z <= max.z {
                pointcloud.points.push(point);
            }
        }

        pointcloud
    }

    pub fn boundaries(&self) -> (Vector3, Vector3) {
        if self.is_empty() {
            return (Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0))
        }
        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in self.iter_points() {
            if point.x < min.x {min.x = point.x;}
            if point.y < min.y {min.y = point.y;}
            if point.z < min.z {min.z = point.z;}

            if point.x > max.x {max.x = point.x;}
            if point.y > max.y {max.y = point.y;}
            if point.z > max.z {max.z = point.z;}
        }

        (min, max)
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn vector_addition_works() {
        let a = Vector3::new(1.0, 1.0, 1.0);
        let b = Vector3::new(1.0, 1.0, 1.0);

        let c = a + b;

        assert_approx_eq!(c.x, 2.0);
        assert_approx_eq!(c.y, 2.0);
        assert_approx_eq!(c.z, 2.0);
    }

    #[test]
    fn vector_subtraction_works() {
        let a = Vector3::new(1.0, 1.0, 1.0);
        let b = Vector3::new(1.0, 1.0, 1.0);

        let c = a - b;

        assert_approx_eq!(c.x, 0.0);
        assert_approx_eq!(c.y, 0.0);
        assert_approx_eq!(c.z, 0.0);
    }

    #[test]
    fn sub_box_works() {
        let mut cloud = PointCloud::new();
        cloud.add_point(Vector3::new(0.0, 0.0, 0.0));
        cloud.add_point(Vector3::new(2.0, 2.0, 2.0));

        let sub_cloud = cloud.sub_box(
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(2.0, 2.0, 2.0));

        assert_eq!(sub_cloud.points.len(), 1);
        assert_eq!(sub_cloud.points[0], Vector3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn pointcloud_boundary_checking_works() {
        let smaller = Vector3::new(0.0, 0.0, 0.0);
        let bigger = Vector3::new(2.0, 2.0, 2.0);
        let mut cloud = PointCloud::new();
        cloud.add_point(smaller);
        cloud.add_point(bigger);

        let (min, max) = cloud.boundaries();

        assert_eq!(min, smaller);
        assert_eq!(max, bigger);
    }

    #[test]
    fn pointcloud_boundary_checking_wont_fail_on_empty() {
        let cloud = PointCloud::new();
        let (min, max) = cloud.boundaries();

        assert_eq!(Vector3::new(0.0, 0.0, 0.0), min);
        assert_eq!(Vector3::new(0.0, 0.0, 0.0), max);
    }

    #[test]
    fn pointcloud_is_empty_checks() {
        let empty_cloud = PointCloud::new();
        let mut full_cloud = PointCloud::new();
        full_cloud.add_point(Vector3::new(0.0, 0.0, 0.0));
        full_cloud.add_point(Vector3::new(2.0, 2.0, 2.0));


        assert!(empty_cloud.is_empty());
        // I feel like ! is too easily missed in this line so I am using == false
        assert!(full_cloud.is_empty() == false);
    }
}
