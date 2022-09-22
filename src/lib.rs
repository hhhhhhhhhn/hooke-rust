mod english;
mod shingle;

pub type Pos = (usize, usize);

#[derive(Clone, Debug)]
pub struct Unit {
    pub value: String,
    pub position: Pos,
    pub index: usize,
}

pub type Text = Vec<Unit>;

pub fn find_matches(a: Text, b: Text) -> Vec<(Unit, Unit)> {
    let matches = a.iter()
        .flat_map(|text_a|
            b.iter()
            .filter(|text_b| text_a.value == text_b.value)
            .map(|text_b| (text_a.clone(), text_b.clone())))
        .collect();
    return matches
}

fn distance((a1, a2): &(Unit, Unit), (b1, b2): &(Unit, Unit)) -> i32 {
    let (ax, ay) = (a1.index as i32, a2.index as i32);
    let (bx, by) = (b1.index as i32, b2.index as i32);
    return (ax - bx).abs().max((ay - by).abs())
}

pub fn cluster(matches: Vec<(Unit, Unit)>, max_distance: i32) -> Vec<Vec<(Unit, Unit)>>{
    let mut clusters: Vec<Vec<(Unit, Unit)>> = Vec::new();
    for matchh in matches {
        let mut in_cluster = false;
        for cluster in clusters.iter_mut() {
            for cluster_match in cluster.iter().clone() { // TODO: Be more efficient
                if distance(&matchh, &cluster_match) <= max_distance {
                    cluster.push(matchh.clone());
                    in_cluster = true;
                    break
                }
            }
        }
        if !in_cluster {
            clusters.push(vec!(matchh))
        }
    }
    return clusters
}

pub fn score_cluster(cluster: &Vec<(Unit, Unit)>) -> usize {
    let text1_start = cluster.iter().map(|x| x.0.index).min().unwrap();
    let text1_end   = cluster.iter().map(|x| x.0.index).max().unwrap();
    let text2_start = cluster.iter().map(|x| x.1.index).min().unwrap();
    let text2_end   = cluster.iter().map(|x| x.1.index).max().unwrap();

    let spread = text1_end - text1_start + text2_end - text2_start;

    return (cluster.len() * cluster.len() * 1024) / spread
}


#[cfg(test)]
mod tests {
    use super::*;

    fn split_words(s: &str) -> Text {
        let s = format!(" {} ", s);
        let iter = s.match_indices(' ').map(|(i, _)| i);
        let mut iter_offset = iter.clone();
        iter_offset.next();
        
        return iter.zip(iter_offset)
            .enumerate()
            .filter_map(|(i, (start, end))|
                        Some(Unit{
                            index: i,
                            position: (start, end),
                            value: s.to_string().get((start+1)..end)?.to_string()
                        }))
            .collect();
    }

    #[test]
    fn test_find_matches() {
        let text1 = split_words("some words I know");
        let text2 = split_words("you know as well as I");
        let matches = find_matches(text1, text2);
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().all(|(x, y)| x.value == y.value));
        assert_eq!(matches.get(0).unwrap().0.value, "I");
        assert_eq!(matches.get(1).unwrap().0.value, "know");
    }

    #[test]
    fn test_cluster() {
        let text1 = split_words("a b c d e f g h i j k l m n o p q r s");
        let text2 = split_words("1 2 c d e f 3 4 5 6 p q 7 r s 8 9 10");
        let matches = find_matches(text1, text2);
        let clusters = cluster(matches, 5);
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters.get(0).unwrap().iter().map(|(x, _)| x.value.clone()).collect::<Vec<String>>(),
                vec!("c".to_string(), "d".to_string(), "e".to_string(), "f".to_string()));
        assert_eq!(clusters.get(1).unwrap().iter().map(|(x, _)| x.value.clone()).collect::<Vec<String>>(),
                vec!("p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()));
    }

    #[test]
    fn test_score() {
        let text1 = split_words("1 2 c d e f 3 4 5 6 p q 7 r s 8 9 10 11 a b c");
        let text2 = split_words("a b c d e f g h i j k l m n o p q r s");
        let matches = find_matches(text1, text2);
        let clusters = cluster(matches, 5); // "c d e f" > "p q 7 r s" > "a b c"
        let scores: Vec<usize> = clusters.iter().map(score_cluster).collect();
        assert!(scores.get(0).unwrap() > scores.get(1).unwrap());
        assert!(scores.get(1).unwrap() > scores.get(2).unwrap());
    }
}
