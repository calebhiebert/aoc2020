use std::collections::HashMap;

const TEST_DATA: &str = "147
174
118
103
67
33
96
28
43
22
16
138
75
148
35
6
10
169
129
115
21
52
58
79
46
7
139
104
91
51
172
57
49
126
95
149
125
123
112
30
78
44
37
167
157
29
173
98
36
63
111
160
18
8
9
159
179
72
110
2
53
150
17
81
97
108
102
56
135
166
168
163
1
25
3
158
101
132
144
45
140
34
156
178
105
68
153
80
82
59
50
122
69
85
109
40
124
119
94
88
13
180
177
133
66
134
60
141";

fn day_10_a(input: &str) -> i64 {
    let mut adapters = input
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    adapters.sort();

    // Add the device, which is 3 higher than the highest joltage
    adapters.push(adapters.last().unwrap() + 3);

    let mut diffs = HashMap::new();

    for (i, a) in adapters.iter().enumerate() {
        let diff;

        if i == 0 {
            diff = *a
        } else {
            diff = adapters[i] - adapters[i - 1]
        }

        if !diffs.contains_key(&diff) {
            diffs.insert(diff, 1);
        } else {
            diffs.insert(diff, diffs.get(&diff).unwrap() + 1);
        }
    }

    println!("{:?}", diffs);

    diffs[&1] * diffs[&3]
}

fn day_10_b(input: &str) -> i64 {
    let mut adapters = input
        .split("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut paths = vec![0; adapters.len()];
    paths[0] = 1;

    for i in 1..adapters.len() {
        let adapter = adapters[i];
        let mut count = 0;

        if i >= 3 && adapter - adapters[i - 3] <= 3 {
            count += paths[i - 3];
        }

        if i >= 2 && adapter - adapters[i - 2] <= 3 {
            count += paths[i - 2];
        }

        if i >= 1 && adapter - adapters[i - 1] <= 3 {
            count += paths[i - 1];
        }

        paths[i] = count;
    }

    paths[adapters.len() - 1]
}

#[test]
fn test_day_10_a() {
    println!("{}", day_10_a(TEST_DATA));
}

#[test]
fn test_day_10_b() {
    println!("{}", day_10_b(TEST_DATA));
}