use std::collections::HashSet;

pub fn balls_and_boxes(balls: &str) -> usize {
    let mut boxes = Vec::new();
    for _ in 0..10 {
        let box_: HashSet<char> = HashSet::new();
        boxes.push(box_);
    }

    let balls: Vec<char> = balls.chars().collect();
    for ball_def in balls.chunks(2) {
        let color = ball_def.first().unwrap();
        let box_idx = ball_def.last().unwrap();
        let box_idx = box_idx.to_string().parse::<usize>().unwrap();
        if let Some(box_) = boxes.get_mut(box_idx) {
            box_.insert(*color);
        }
    }

    boxes
        .iter()
        .filter(|box_| {
            let mut ball_colors = 0;
            if box_.get(&'R').is_some() {
                ball_colors += 1;
            }
            if box_.get(&'B').is_some() {
                ball_colors += 1;
            }
            if box_.get(&'G').is_some() {
                ball_colors += 1;
            }
            ball_colors == 3
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = balls_and_boxes("G0B1R2R0B0");
        assert_eq!(result, 1);
    }

    #[test]
    fn test2() {
        let result = balls_and_boxes("G1R3R6B3G6B1B6R1G3");
        assert_eq!(result, 3);
    }

    #[test]
    fn test3() {
        let result = balls_and_boxes("B3B2G1B3");
        assert_eq!(result, 0);
    }
}
