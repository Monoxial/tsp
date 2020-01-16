#![feature(vec_remove_item)]

mod point;

use point::Point;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        eprintln!("Error in argument(s)! Use : {} 'fileLocation' ", &arguments[0]);
    } else {
        let files = file_reader(&arguments[1]); //Contain point from file (string)
        let points = file_parser(&files); //Contain points in type "point"

        
        let result = two_opt(nearest_neighbour(points)); //Result from "nearest_neighbour" for best tour length

        file_write(result);
    }
}


fn file_reader(location: &str) -> Vec<String> {
    // Open the file in read-only
    let file = File::open(location).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec!();

    // Read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        lines.push(line.unwrap().parse::<String>().unwrap());
    }
    lines.remove(0); //Delete number of elements before return
    lines
}

fn file_parser(lines: &[String]) -> Vec<Point> {
    let mut coord: Vec<Point> = vec![];

    for line in lines {
        let mut iterator = line.split_whitespace();
        coord.push(Point::new(iterator.next().unwrap().parse::<f64>().unwrap(), iterator.next().unwrap().parse::<f64>().unwrap()));
    } //For each line create a point 
    coord
}

fn file_write(points: Vec<Point>) {
    let mut file_to_export = std::fs::File::create(points.len().to_string() + ".out3").expect("create failed");
    file_to_export.write_all(points.len().to_string().as_bytes()).expect("write failed");
    file_to_export.write_all(b"\n").expect("write failed");
    file_to_export.write_all(tour_length(&points).to_string().as_bytes()).expect("write failed");
    file_to_export.write_all(b"\n").expect("write failed");
    for point in points {
        file_to_export.write_all(point.get_x().to_string().as_bytes()).expect("write failed");
        file_to_export.write_all(b" ").expect("write failed");
        file_to_export.write_all(point.get_y().to_string().as_bytes()).expect("write failed");
        file_to_export.write_all(b"\n").expect("write failed");
    }
}

fn tour_length(points: &Vec<Point>) -> f64 {
    let mut result: f64 = 0.0;
    let mut previous = points[points.len() - 1];

    for point in points {
        result += point.distance(&previous);
        previous = *point;
    }

    result
}

fn nearest_neighbour(mut points: Vec<Point>) -> Vec<Point> {
    let mut result: Vec<Point> = vec!();
    let mut current_point = points.remove(0);
    let mut closest_point = points[0];

    result.push(current_point);

    while points.len() > 0 {
        let mut distance = std::f64::INFINITY;
        for count in 0..points.len() { //Find the nearest neighbour for the given point
            if current_point.distance(&points[count]) < distance {
                distance = current_point.distance(&points[count]);
                closest_point = points[count];
            }
        }
        current_point = closest_point;
        points.remove_item(&closest_point);
        result.push(current_point);
    }
    result
}


fn two_opt(mut points: Vec<Point>) -> Vec<Point> {
    let mut new_tour: Vec<Point>;
    let mut best_distance = tour_length(&points);
    let mut new_distance: f64;

    let mut swaps = true;
    while swaps != false {
        swaps = false;

        for i in 1..points.len() - 2 {
            for j in i+1..points.len() - 1 {
                //check distance of line 1,2 + line 3,4 against 1,3 + 2,4 if there is improvement, call two_opt_swap

                if (points[i].distance(&points[i - 1]) + points[j + 1].distance(&points[j])) >=
                    (points[i].distance(&points[j + 1]) + points[i - 1].distance(&points[j]))
                {
                    new_tour = two_opt_swap(&points, i, j);

                    new_distance = tour_length(&new_tour);

                    if new_distance < best_distance {
                        points = new_tour;
                        best_distance = new_distance;
                        swaps = true;
                    } //If a best distance found, launch a new swaps
                }
            }
        }
    }

    points
}

fn two_opt_swap(points: &Vec<Point>, i: usize, j: usize) -> Vec<Point> {

    let mut new_tour: Vec<Point> = vec!();

    //inverting the order of the points
    for c in 0..i {
        new_tour.push(points[c]);
    }


    //invert order between 2 passed points i and j
    let mut dec = 0;
    for _c in i-1..j {
        new_tour.push(points[j - dec]);
        dec=dec+1;
    }

    //add the remain points
    let size = points.len();
    for c in j + 1..size {
        new_tour.push(points[c]);
    }

    new_tour
}
