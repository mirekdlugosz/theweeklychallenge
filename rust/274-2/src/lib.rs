use std::collections::HashMap;

// 1. How often bus departs (assumed to be a divider of 60)
// 2. First minute in hour when bus departs (assumed to be lesser than 1.)
// 3. How long the route takes
pub struct RouteDefinition(usize, usize, usize);

fn parse_route(definition: &RouteDefinition) -> Vec<(usize, usize)> {
    let frequency = definition.0;
    let initial = definition.1;
    let time_needed = definition.2;

    let mut output: Vec<(usize, usize)> = Vec::with_capacity(60 / frequency);

    let mut current = initial;

    loop {
        if current >= 60 + initial + frequency {
            break;
        }
        output.push((current, current + time_needed));
        current += frequency;
    }

    output
}

pub fn bus_route(definitions: &[RouteDefinition]) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(60);
    let mut departures: HashMap<usize, Vec<usize>> = HashMap::with_capacity(60);

    // create a map where keys are departure times (minute within hour) and value
    // is a list of arrival times. This needs to be a list, because technically
    // multiple buses may depart at the same time.
    for route_definition in definitions {
        let parsed_route = parse_route(route_definition);
        for departure in parsed_route {
            let departure_time = departure.0;
            let arrival_time = departure.1;

            departures.entry(departure_time)
                .or_default()
                .push(arrival_time);
        }
    }

    // vector of times when any bus departs
    let mut departures_keys: Vec<&usize> = departures.keys().collect();
    departures_keys.sort_unstable();

    // for each minute within hour
    // 1. find next departure and one after it
    // 2. get the minimum time in arrival times of both
    // 3. if arrival time after is smaller than arrival time
    //    of next one, then it's better to wait for next bus
    for minute in 0..=59 {
        let mut possible_departures = departures_keys
            .iter()
            .filter(|t| ***t >= minute);
        let closest_departure_t = match possible_departures.next() {
            Some(t) => t,
            None => break,
        };
        let next_departure_t = match possible_departures.next() {
            Some(t) => t,
            None => break,
        };

        let closest_departure_a = departures
            .get(*closest_departure_t)
            .unwrap()
            .iter()
            .min()
            .unwrap();

        let next_departure_a = departures
            .get(*next_departure_t)
            .unwrap()
            .iter()
            .min()
            .unwrap();

        if closest_departure_a > next_departure_a {
            output.push(minute);
        } 
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let definitions = vec!(
            RouteDefinition(12, 11, 41),
            RouteDefinition(15, 5, 35),
        );
        let result = vec!(36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47);
        assert_eq!(bus_route(&definitions), result);
    }

    #[test]
    fn test2() {
        let definitions = vec!(
            RouteDefinition(12, 3, 41),
            RouteDefinition(15, 9, 35),
            RouteDefinition(30, 5, 25),
        );
        let result = vec!(0, 1, 2, 3, 25, 26, 27, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 55, 56, 57, 58, 59);
        assert_eq!(bus_route(&definitions), result);
    }
}
