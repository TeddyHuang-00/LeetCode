pub struct Solution;

impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let nums = s
            .chars()
            .step_by(2)
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        let ops = s
            .chars()
            .skip(1)
            .step_by(2)
            .map(|c| match c {
                '+' => false,
                '*' => true,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        // Get all possible values of the expression
        let mut dp = vec![vec![std::collections::HashSet::new(); nums.len()]; nums.len()];
        (0..nums.len()).for_each(|i| {
            dp[i][i].insert(nums[i]);
        });
        for len in 1..nums.len() {
            for (i, j) in (0..nums.len()).zip(len..nums.len()) {
                dp[i][j] = (i..j)
                    .flat_map(|k| {
                        if ops[k] {
                            dp[i][k]
                                .iter()
                                .flat_map(|&x| dp[k + 1][j].iter().map(move |&y| x * y))
                                .filter(|&x| x <= 1000)
                                .collect::<std::collections::HashSet<_>>()
                        } else {
                            dp[i][k]
                                .iter()
                                .flat_map(|&x| dp[k + 1][j].iter().map(move |&y| x + y))
                                .filter(|&x| x <= 1000)
                                .collect::<std::collections::HashSet<_>>()
                        }
                    })
                    .collect();
            }
        }
        let dp = dp[0].last().unwrap();
        // Get correct value by evaluating the expression
        let mut correct = Vec::with_capacity(nums.len());
        correct.push(nums[0]);
        for (&op, &num) in ops.iter().zip(nums.iter().skip(1)) {
            if op {
                *correct.last_mut().unwrap() *= num;
            } else {
                correct.push(num);
            }
        }
        let correct = correct.into_iter().sum();
        let mut counter = std::collections::HashMap::new();
        answers
            .iter()
            .for_each(|&ans| *counter.entry(ans).or_insert(0) += 1);
        counter
            .iter()
            .map(|(&ans, &cnt)| {
                if ans == correct {
                    5 * cnt
                } else if dp.contains(&ans) {
                    2 * cnt
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::score_of_students(String::from("7+3*1*2"), vec![20, 13, 42]),
            7
        );
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::score_of_students(String::from("3+5*2"), vec![13, 0, 10, 13, 13, 16, 16]),
            19
        );
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::score_of_students(String::from("6+0*1"), vec![12, 9, 6, 4, 8, 6]),
            10
        );
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::score_of_students(
                String::from("4+6+2+8*6+6*8+2*8+6*4+4*6+6*6+8"),
                vec![
                    774, 216, 914, 476, 798, 216, 216, 479, 442, 106, 216, 216, 558, 216, 652, 356,
                    920, 950, 216, 216, 630, 216, 974, 216, 570, 506, 634, 420, 478, 216, 372, 216,
                    800, 848, 326, 455, 216, 536, 546, 790, 426, 41, 726, 459, 216, 624, 216, 256,
                    610, 247, 226, 456, 268, 271, 216, 692, 502, 216, 71, 696, 216, 216, 643, 216,
                    594, 714, 820, 216, 216, 638, 421, 239, 236, 216, 216, 216, 924, 570, 418, 212,
                    6, 216, 216, 216, 216, 216, 6, 216, 749, 745, 716, 216, 761, 862, 216, 216,
                    216, 216, 398, 428, 438, 816, 960, 249, 630, 896, 154, 764, 216, 216, 382, 216,
                    764, 216, 443, 216, 841, 792, 216, 322, 30, 216, 344, 962, 834, 640, 436, 526,
                    990, 216, 216, 811, 216, 594, 216, 531, 512, 659, 216, 216, 594, 216, 980, 904,
                    388, 216, 392, 866, 672, 863, 796, 176, 479, 526, 216, 842, 93, 216, 216, 896,
                    216, 216, 372, 404, 216, 216, 216, 836, 216, 322, 352, 216, 216, 25, 710, 216,
                    782, 779, 871, 216, 216, 782, 216, 814, 568, 216, 216, 372, 886, 688, 852, 528,
                    216, 245, 216, 994, 216, 759, 912, 532, 216, 35, 216, 760, 412, 216, 624, 216,
                    856, 216, 730, 216, 624, 716, 216, 216, 502, 216, 628, 784, 245, 267, 216, 216,
                    361, 336, 866, 634, 392, 318, 62, 844, 216, 820, 371, 733, 216, 426, 404, 892,
                    216, 946, 900, 325, 883, 216, 664, 723, 216, 506, 942, 216, 558, 242, 216, 918,
                    216, 964, 61, 621, 216, 216, 304, 472, 696, 289, 718, 638, 216, 216, 566, 564,
                    498, 382, 216, 556, 216, 396, 956, 548, 216, 710, 216, 404, 216, 518, 558, 216,
                    945, 670, 216, 722, 216, 216, 622, 216, 546, 430, 352, 571, 311, 508, 216, 436,
                    988, 999, 483, 394, 216, 686, 216, 404, 373, 145, 638, 554, 872, 216, 736, 63,
                    281, 552, 682, 765, 1, 92, 802, 216, 216, 631, 687, 619, 216, 475, 216, 340,
                    725, 216, 792, 216, 558, 1000, 596, 216, 300, 216, 216, 306, 393, 311, 216,
                    137, 216, 216, 542, 216, 745, 798, 216, 768, 706, 216, 108, 216, 216, 216, 642,
                    53, 216, 784, 216, 216, 216, 679, 936, 922, 714, 412, 216, 216, 49, 720, 306,
                    216, 347, 216, 216, 551, 216, 216, 731, 385, 373, 692, 216, 216, 49, 216, 216,
                    216, 276, 328, 297, 216, 216, 216, 388, 206, 762, 226, 392, 173, 408, 870, 216,
                    216, 652, 216, 216, 216, 338, 616, 216, 844, 844, 395, 973, 216, 216, 281, 216,
                    836, 858, 216, 117, 768, 906, 280, 344, 976, 432, 287, 216, 996, 884, 740, 652,
                    552, 495, 766, 216, 216, 539, 216, 578, 216, 7, 528, 216, 947, 840, 316, 800,
                    504, 727, 564, 940, 216, 918, 665, 216, 712, 216, 216, 610, 216, 832, 398, 216,
                    525, 975, 534, 442, 216, 654, 216, 172, 620, 280, 408, 734, 788, 216, 80, 216,
                    924, 996, 602, 216, 680, 366, 27, 216, 650, 216, 216, 960, 216, 207, 604, 509,
                    590, 216, 648, 552, 216, 482, 365, 578, 834, 546, 216, 216, 978, 546, 562, 912,
                    454, 216, 616, 786, 872, 947, 216, 118, 216, 553, 816, 376, 873, 978, 950, 240,
                    885, 216, 912, 999, 216, 852, 216, 216, 790, 904, 328, 580, 66, 216, 351, 139,
                    216, 216, 254, 754, 297, 216, 346, 266, 598, 844, 216, 216, 605, 569, 518, 947,
                    708, 216, 216, 312, 518, 674, 9, 676, 516, 150, 216, 892, 216, 460, 503, 264,
                    422, 74, 726, 216, 216, 216, 760, 322, 594, 979, 338, 958, 593, 564, 80, 671,
                    785, 216, 364, 216, 752, 216, 562, 216, 264, 376, 216, 754, 216, 216, 740, 216,
                    822, 216, 279, 278, 588, 536, 216, 960, 852, 783, 768, 216, 356, 274, 825, 226,
                    717, 966, 634, 216, 233, 932, 566, 216, 216, 686, 296, 133, 320, 508, 536, 916,
                    200, 216, 340, 216, 554, 412, 994, 550, 777, 104, 216, 758, 216, 216, 666, 984,
                    216, 586, 203, 568, 216, 370, 850, 600, 916, 216, 958, 994, 346, 216, 690, 21,
                    812, 604, 216, 454, 670, 244, 216, 834, 216, 824, 216, 216, 534, 800, 323, 426,
                    216, 235, 216, 216, 970, 288, 555, 216, 362, 212, 420, 216, 256, 988, 736, 127,
                    216, 216, 216, 216, 542, 368, 216, 646, 842, 908, 216, 492, 216, 216, 8, 216,
                    61, 506, 584, 216, 216, 216, 927, 216, 482, 216, 546, 986, 970, 552, 216, 214,
                    600, 216, 472, 179, 98, 388, 254, 216, 536, 962, 240, 216, 794, 540, 764, 216,
                    597, 854, 334, 396, 80, 900, 909, 492, 940, 347, 920, 834, 726, 89, 844, 320,
                    216, 566, 676, 616, 776, 216, 806, 752, 300, 576, 197, 216, 910, 408, 50, 744,
                    188, 454, 216, 876, 216, 864, 264, 73, 504, 966, 216, 838, 360, 216, 296, 945,
                    674, 689, 924, 562, 666, 216, 290, 566, 480, 472, 216, 216, 515, 761, 984, 216,
                    906, 336, 938, 775, 758, 361, 216, 844, 914, 780, 511, 444, 38, 640, 112, 477,
                    460, 944, 216, 216, 934, 216, 386, 216, 216, 625, 818, 216, 216, 676, 216, 578,
                    626, 482, 134, 926, 350, 976, 116, 932, 213, 844, 950, 709, 758, 216, 216, 846,
                    216, 494, 822, 216, 840, 202, 666, 216, 394, 888, 814, 318, 216, 858, 942, 836,
                    228, 216, 216, 216, 161, 700, 458, 730, 388, 177, 969, 216, 648, 798, 216, 338,
                    765, 216, 754, 838, 216, 714, 820, 216, 216, 570, 361, 216, 966, 216, 216, 815,
                    216, 966, 306, 209, 872, 662, 603, 398, 428, 712, 962, 216, 216, 997, 216, 824,
                    216, 216, 656, 493, 782, 364, 216, 185, 378, 680, 448, 216, 842, 244, 724, 826,
                    303, 862, 813, 850, 216, 818, 47, 537, 216, 492, 533, 265, 216, 846, 216, 516,
                    216, 886, 216, 806, 696, 352, 719, 426, 216, 388, 216, 216, 756, 216, 274, 216,
                    482, 730, 608, 982, 239, 493, 556, 516, 632, 216, 811, 372, 216, 216, 358, 566,
                    8, 370, 996, 508, 972, 568, 320, 766, 891, 216, 570, 760, 656, 860, 340, 216,
                    488, 216, 812, 198, 216, 304, 700, 527, 737, 990, 776, 216, 640, 957, 562, 619,
                    868, 854, 135, 216, 486, 344, 343, 866, 912, 502, 464, 676, 858, 692, 760, 358,
                    210, 442, 730, 216, 216, 922, 216, 539, 216, 957, 658, 870, 274, 211, 484, 556,
                    216, 586, 216, 654, 241, 910, 216, 296, 216, 762, 646, 53, 387, 981, 740, 320,
                    532, 734, 584, 916, 216, 216, 181, 768, 644, 216, 832, 990, 216, 216, 430, 724,
                    216, 171, 414, 526, 216, 328, 216, 849, 455, 216, 356, 352, 820, 614, 580, 103,
                    216, 706, 444, 944, 694, 910, 296, 216, 758, 328, 848, 326, 376, 216, 646, 216,
                    868, 177, 95, 784, 216, 694, 185, 216, 216, 360, 416, 216, 910, 951, 216, 948,
                    114, 216, 728, 233, 10, 395, 216, 352, 44, 698, 205, 492, 216, 312, 216, 216,
                    216, 378, 216, 216, 216, 216, 216, 598, 690, 70, 744, 304, 256, 76, 216, 628,
                    878, 590, 984, 216, 766, 420, 774, 734, 448, 528, 593, 216, 302, 582, 438, 215,
                    326, 216, 628, 216, 842, 216, 45, 765, 216, 159, 722, 216, 902, 780, 365, 679,
                    774, 851, 710, 940, 511, 798, 314, 264, 502, 89, 324, 216, 460, 360, 386, 548,
                    216, 216, 804, 592, 216, 539, 451, 936, 468, 105, 391, 42, 114, 216, 263, 790,
                    100, 617, 221, 881, 978, 910, 368, 834, 772, 29, 216, 216, 370, 216, 852, 896,
                    216, 870, 972, 250, 216, 20, 970, 422, 648, 152, 780, 974, 216, 534, 979, 798,
                    216, 652, 216, 542, 278, 216, 216, 860, 216, 563, 454, 102, 800, 216, 964, 360,
                    216, 866, 216, 910, 216, 761, 572, 946, 216, 216, 797, 994, 454, 216, 216, 548,
                    98, 216, 216, 932, 616, 490, 97, 986, 216, 933, 573, 216, 502, 816, 536, 610,
                    226, 216, 216, 336, 216, 964, 73, 216, 216, 598, 216, 216, 384, 349, 216, 730,
                    222, 934, 476, 216, 702, 538, 216, 834, 216, 946, 216, 964, 216, 820, 427, 256,
                    740, 544, 474, 600, 400, 47, 896, 834, 76, 980, 216, 216, 216, 645, 888, 482,
                    216, 766, 216, 956, 536, 216, 400, 432, 209, 216, 200, 658, 964, 216, 722, 156,
                    216, 442, 808, 890, 216, 912, 473, 940, 840, 992, 216, 767, 762, 898, 750, 216,
                    216, 216, 216, 348, 908, 216, 216, 964, 674, 999, 416, 633, 806, 3, 3, 516,
                    328, 748, 216, 216, 216, 566, 810, 412, 603, 216, 216, 962, 435, 216, 94, 386,
                    216, 836, 562, 446, 984, 438, 879, 976, 370, 216, 216, 613, 216, 930, 546, 216,
                    216, 838, 216, 966, 424, 564, 216, 266, 216, 372, 384, 567, 216, 381, 192, 216,
                    456, 386, 372, 730, 568, 632, 346, 962, 873, 216, 950, 864, 216, 241, 778, 369,
                    653, 312, 950, 216, 216, 216, 972, 438, 216, 851, 472, 834, 216, 978, 216, 82,
                    190, 496, 154, 968, 816, 216, 328, 604, 842, 880, 278, 314, 216, 932, 716, 233,
                    216, 426, 216, 700, 216, 496, 980, 216, 903, 253, 654, 216, 362, 764, 512, 765,
                    338, 482, 382, 484, 441, 346, 518, 579, 622, 878, 568, 940, 602, 722, 216, 752,
                    216, 216, 660, 216, 216, 696, 216, 430, 221, 876, 216, 216, 149, 608, 781, 776,
                    216, 240, 221, 716, 216, 774, 958, 80, 714, 574, 626, 552, 772, 754, 216, 333,
                    740, 216, 792, 674, 660, 684, 216, 896, 792, 934, 264, 788, 286, 118, 550, 719,
                    892, 802, 216, 580, 802, 416, 216, 478, 319, 949, 234, 290, 216, 216, 216, 540,
                    216, 118, 947, 928, 356, 125, 808, 216, 216, 108, 938, 264, 552, 666, 216, 813,
                    832, 289, 966, 924, 922, 52, 482, 216, 686, 906, 598, 907, 697, 346, 216, 395,
                    2, 810, 769, 980, 584, 114, 906, 216, 216, 358, 216, 216, 274, 788, 216, 764,
                    762, 274, 996, 730, 716, 216, 698, 216, 929, 588, 216, 351, 554, 828, 946, 288,
                    634, 736, 574, 834, 198, 216, 404, 458, 216, 694, 445, 795, 716, 434, 216, 670,
                    216, 896, 479, 252, 563, 404, 628, 648, 872, 216, 250, 216, 784, 812, 959, 965,
                    29, 824, 366, 678, 492, 282, 530, 786, 216, 216, 986, 728, 953, 862, 216, 216,
                    759, 614, 388, 806, 216, 216, 760, 850, 216, 267, 216, 216, 358, 216, 252, 882,
                    216, 766, 507, 626, 285, 534, 630, 216, 443, 688, 216, 779, 984, 534, 172, 346,
                    564, 274, 492, 216, 580, 792, 216, 216, 804, 967, 216, 782, 216, 620, 830, 216,
                    120, 936, 546, 844, 335, 216, 922, 216, 216, 752, 202, 928, 216, 216, 216, 582,
                    644, 976, 608, 124, 216, 162, 918, 968, 6, 478, 216, 796, 216, 780, 216, 890,
                    776, 556, 588, 752, 216, 684, 247, 794, 918, 836, 216, 216, 575, 256, 24, 358,
                    216, 216, 216, 216, 216, 216, 240, 216, 548, 368, 216, 450, 726, 46, 764, 931,
                    930, 959, 78, 216, 583, 532, 262, 216, 372, 216, 216, 588, 445, 920, 623, 216,
                    0, 870, 879, 820, 216, 216, 700, 778, 685, 878, 461, 34, 300, 216, 464, 485,
                    216, 155, 266, 908, 352, 612, 216, 944, 216, 266, 938, 394, 788, 504, 405, 216,
                    672, 216, 320, 533, 216, 216, 94, 334, 490, 318, 216, 387, 654, 962, 585, 216,
                    216, 360, 216, 216, 498, 800, 216, 216, 389, 216, 718, 99, 794, 216, 856, 216,
                    874, 376, 216, 788, 428, 984, 174, 216, 216, 532, 216, 216, 528, 730, 298, 216,
                    216, 143, 639, 216, 616, 310, 831, 216, 984, 832, 216, 392, 216, 370, 216, 188,
                    594, 876, 216, 396, 642, 216, 752, 216, 650, 912, 147, 216, 876, 648, 582, 216,
                    216, 216, 276, 216, 216, 970, 216, 906, 216, 622, 818, 992, 665, 692, 970, 216,
                    216, 216, 216, 636, 490, 980, 216, 216, 216, 216, 802, 216, 216, 728, 215, 216,
                    558, 116, 216, 578, 216, 724, 216, 950, 346, 467, 870, 216, 216, 432, 984, 216,
                    446, 420, 216, 891, 216, 738, 216, 560, 216, 579, 548, 588, 461, 686, 216, 902,
                    216, 464, 811, 532, 966, 700, 216, 240, 910, 804, 216, 216, 216, 854, 904, 802,
                    216, 216, 216, 532, 856, 216, 382, 577, 216, 433, 870, 216, 584, 216, 894, 216,
                    828, 382, 593, 878, 664, 39, 772, 844, 64, 970, 48, 372, 930, 473, 952, 216,
                    492, 216, 762, 974, 256, 478, 216, 948, 216, 216, 476, 507, 548, 610, 14, 216,
                    216, 216, 682, 658, 172, 216, 338, 740, 980, 352, 216, 808, 7, 564, 810, 216,
                    994, 322, 678, 770, 654, 216, 796, 216, 724, 216, 216, 986, 143, 648, 996, 216,
                    604, 216, 577, 696, 660, 216, 581, 891, 351, 538, 216, 750, 558, 288, 216, 974,
                    598, 216, 660, 667, 216, 842, 664, 508, 950, 346, 216, 610, 936, 608, 952, 396,
                    216, 674, 938, 169, 442, 596, 428, 638, 216, 506, 216, 926, 888, 366, 35, 698,
                    814, 159, 177, 642, 214, 216, 995, 449, 336, 898, 972, 433, 968, 652, 770, 584,
                    802, 466, 216, 428, 422, 956, 653, 216, 796, 644, 716, 454, 682, 743, 802, 216,
                    276, 594, 280, 216, 216, 936, 216, 190, 280, 703, 694, 216, 468, 216, 216, 216,
                    604, 436, 216, 593, 874, 656, 414, 433, 216, 216, 654, 216, 216, 216, 888, 792,
                    216, 186, 334, 216, 103, 612, 910, 216, 261, 756, 660, 216, 581, 765, 216, 216,
                    867, 462, 684, 718, 216, 307, 738, 216, 216, 216, 216, 304, 914, 348, 216, 216,
                    216, 786, 109, 648, 216, 866, 442, 216, 216, 240, 444, 216, 17, 216, 56, 224,
                    663, 744, 216, 694, 628, 118, 647, 385, 976, 216, 216, 862, 216, 614, 216, 508,
                    267, 606, 216, 740, 216, 492, 216, 842, 922, 694, 216, 216, 299, 884, 216, 216,
                    216, 472, 322, 898, 542, 536, 902, 944, 748, 216, 566, 216, 216, 360, 318, 216,
                    872, 630, 216, 956, 435, 216, 249, 512, 305, 58, 506, 934, 216, 750, 464, 680,
                    470, 216, 763, 806, 348, 645, 216, 308, 754, 756, 216, 216, 966, 944, 852, 13,
                    216, 216, 946, 216, 872, 216, 764, 587, 964, 97, 216, 718, 754, 528, 704, 216,
                    206, 959, 920, 384, 216, 1000, 245, 728, 216, 592, 472, 216, 159, 364, 216,
                    406, 876, 216, 722, 532, 201, 700, 74, 216, 466, 316, 216, 216, 213, 850, 216,
                    216, 428, 876, 216, 553, 496, 216, 216, 808, 512, 216, 216, 143, 216, 834, 466,
                    216, 592, 768, 115, 670, 988, 378, 216, 216, 216, 956, 264, 216, 984, 216, 544,
                    272, 300, 404, 967, 216, 216, 687, 23, 958, 318, 15, 888, 617, 588, 292, 706,
                    216, 216, 216, 551, 826, 976, 862, 818, 328, 762, 865, 996, 10, 839, 216, 426,
                    216, 216, 684, 216, 190, 348, 214, 216, 683, 197, 216, 329, 144, 943, 876, 216,
                    844, 460, 398, 740, 645, 460, 216, 628, 344, 432, 216, 216, 831, 710, 266, 120,
                    436, 55, 576, 216, 850, 216, 216, 216, 216, 628, 216, 663, 216, 321, 310, 990,
                    138, 672, 468, 216, 882, 216, 340, 998, 356, 963, 268, 648, 709, 370, 216, 216,
                    804, 216, 836, 346, 216, 906, 684, 565, 216, 848, 216, 385, 955, 216, 942, 472,
                    473, 334, 216, 216, 414, 392, 808, 768, 521, 216, 216, 951, 632, 283, 446, 903,
                    216, 512, 316, 246, 216, 216, 216, 207, 280, 129, 190, 216, 216, 216, 216, 216,
                    446, 926, 216, 904, 690, 692, 376, 476, 216, 678, 216, 634, 944, 515, 226, 216,
                    686, 370, 216, 966, 736, 846, 685, 48, 513, 11, 929, 912, 216, 216, 745, 768,
                    538, 746, 886, 706, 216, 492, 216, 216, 996, 643, 792, 216, 216, 325, 216, 679,
                    216, 53, 192, 333, 216, 480, 798, 636, 836, 85, 570, 354, 412, 816, 594, 684,
                    924, 858, 872, 173, 370, 624, 938, 216, 768, 800, 562, 216, 820, 866, 868, 456,
                    320, 987, 428, 602, 201, 956, 26, 872, 216, 183, 970, 216, 446, 216, 684, 770,
                    492, 216, 682, 804, 216, 765, 436, 654, 216, 334, 216, 399, 772, 316, 706, 248,
                    263, 312, 216, 840, 966, 216, 318, 674, 314, 697, 216, 908, 514, 739, 216, 746,
                    216, 216, 216, 216, 620, 754, 216, 363, 299, 216, 404, 216, 432, 216, 323, 900,
                    842, 39, 718, 216, 216, 402, 146, 429, 216, 215, 456, 604, 774, 408, 544, 935,
                    216, 516, 503, 731, 262, 440, 906, 216, 430, 744, 828, 874, 796, 844, 216, 786,
                    216, 338, 372, 821, 216, 830, 216, 196, 216, 942, 954, 216, 822, 398, 918, 216,
                    753, 992, 994, 659, 375, 800, 668, 216, 209, 172, 547, 296, 542, 606, 216, 910,
                    664, 78, 426, 664, 216, 870, 352, 216, 216, 216, 216, 216, 834, 216, 216, 216,
                    216, 822, 917, 704, 766, 991, 121, 287, 713, 588, 918, 216, 538, 792, 724, 650,
                    216, 614, 216, 229, 946, 216, 346, 821, 914, 153, 686, 216, 594, 482, 216, 216,
                    818, 344, 278, 342, 834, 498, 216, 216, 216, 794, 208, 216, 890, 216, 216, 216,
                    216, 216, 976, 216, 672, 311, 216, 659, 336, 513, 700, 700, 216, 946, 826, 652,
                    858, 884, 216, 347, 924, 216, 660, 622, 216, 886, 768, 216, 240, 416, 788, 424,
                    216, 216, 216, 110, 216, 216, 616, 139, 73, 216, 734, 216, 810, 216, 700, 211,
                    344, 216, 336, 894, 718, 216, 383, 456, 216, 216, 722, 570, 47, 780, 794, 296,
                    650, 646, 216, 534, 568, 426, 151, 216, 682, 636, 789, 816, 99, 286, 629, 956,
                    216, 216, 216, 216, 74, 216, 411, 632, 900, 217, 728, 852, 216, 554, 946, 223,
                    640, 906, 216, 986, 216, 742, 216, 926, 216, 325, 442, 865, 961, 174, 216, 740,
                    820, 446, 216, 510, 492, 216, 958, 793, 700, 470, 264, 858, 216, 314, 61, 216,
                    416, 216, 216, 674, 924, 216, 764, 514, 328, 216, 580, 216, 216, 216, 766, 216,
                    216, 532, 885, 216, 216, 686, 702, 991, 802, 992, 516, 958, 216, 300, 428, 970,
                    982, 642, 742, 902, 386, 460, 424, 672, 774, 668, 216, 216, 101, 264, 58, 281,
                    18, 481, 198, 700, 216, 216, 318, 216, 310, 216, 87, 395, 216, 402, 620, 904,
                    387, 476, 746, 71, 816, 216, 61, 216, 216, 216, 688, 364, 216, 216, 216, 826,
                    422, 233, 842, 942, 70, 216, 922, 173, 596, 216, 226, 173, 216, 644, 216, 588,
                    362, 447, 564, 955, 344, 223, 80, 1000, 878, 216, 760, 882, 216, 318, 352, 976,
                    790, 216, 296, 962, 963, 216, 216, 790, 567, 217, 216, 216, 216, 448, 322, 211,
                    921, 870, 913, 998, 216, 787, 674, 216, 874, 216, 323, 386, 520, 534, 216, 216,
                    907, 216, 594, 518, 638, 644, 868, 216, 720, 80, 456, 288, 216, 778, 269, 550,
                    855, 216, 216, 732, 395, 216, 184, 905, 627, 662, 696, 216, 468, 984, 216, 509,
                    216, 216, 485, 572, 216, 868, 272, 216, 216, 216, 364, 216, 216, 127, 472, 376,
                    792, 216, 607, 840, 216, 277, 758, 924, 392, 882, 354, 672, 816, 216, 584, 576,
                    762, 487, 186, 216, 994, 890, 418, 216, 494, 458, 104, 992, 216, 216, 782, 990,
                    969, 216, 207, 716, 216, 216, 620, 848, 216, 926, 276, 216, 968, 7, 322, 814,
                    550, 216, 793, 316, 306, 248, 216, 216, 216, 216, 216, 216, 216, 297, 756, 844,
                    216, 216, 578, 548, 186, 290, 216, 298, 391, 216, 782, 458, 943, 964, 216, 639,
                    216, 216, 300, 216, 94, 739, 994, 846, 784, 216, 643, 563, 216, 216, 33, 572,
                    216, 216, 826, 60, 222, 216, 743, 622, 231, 398, 872, 225, 544, 216, 880, 216,
                    882, 772, 113, 837, 216, 338, 871, 258, 216, 636, 489, 216, 576, 810, 816, 924,
                    322, 197, 998, 482, 216, 384, 216, 216, 352, 216, 216, 216, 617, 388, 967, 216,
                    786, 216, 216, 216, 216, 149, 368, 878, 982, 429, 216, 216, 274, 778, 89, 392,
                    508, 216, 216, 615, 562, 636, 216, 504, 897, 216, 951, 488, 216, 216, 678, 684,
                    216, 724, 216, 404, 811, 399, 581, 216, 216, 185, 681, 896, 990, 670, 216, 472,
                    256, 646, 76, 216, 378, 893, 752, 598, 528, 482, 216, 216, 216, 586, 294, 216,
                    216, 216, 766, 316, 596, 312, 750, 514, 216, 642, 706, 614, 849, 968, 873, 386,
                    216, 291, 724, 216, 216, 216, 120, 384, 216, 216, 802, 216, 304, 216, 471, 504,
                    324, 55, 576, 180, 954, 216, 216, 698, 920, 690, 216, 216, 266, 216, 300, 328,
                    216, 216, 216, 346, 992, 986, 696, 216, 50, 216, 519, 216, 556, 814, 755, 308,
                    239, 22, 958, 216, 216, 216, 216, 773, 216, 330, 347, 216, 364, 906, 684, 216,
                    823, 216, 852, 97, 826, 396, 550, 446, 216, 216, 852, 980, 216, 44, 216, 871,
                    203, 966, 216, 327, 216, 216, 216, 216, 216, 836, 774, 394, 216, 216, 720, 820,
                    800, 798, 322, 628, 240, 216, 216, 285, 100, 306, 954, 914, 864, 560, 216, 985,
                    654, 590, 816, 658, 216, 56, 570, 29, 123, 216, 216, 587, 237, 758, 900, 782,
                    705, 534, 216, 419, 216, 391, 216, 938, 449, 916, 216, 710, 6, 323, 674, 555,
                    426, 734, 796, 568, 216, 534, 72, 358, 592, 912, 216, 216, 982, 910, 548, 235,
                    693, 696, 216, 233, 598, 216, 999, 280, 786, 216, 932, 216, 938, 216, 818, 616,
                    216, 216, 216, 216, 167, 999, 802, 596, 874, 265, 688, 216, 324, 216, 571, 774,
                    859, 76, 119, 874, 216, 216, 962, 906, 216, 768, 216, 216, 216, 216, 624, 872,
                    702, 932, 192, 216, 820, 216, 357, 216, 904, 216, 880, 793, 216, 216, 216, 216,
                    544, 684, 216, 662, 216, 586, 757, 216, 438, 216, 216, 732, 550, 848, 412, 336,
                    216, 826, 216, 924, 496, 216, 766, 216, 690, 346, 184, 676, 844, 216, 216, 216,
                    216, 652, 588, 346, 216, 646, 216, 676, 722, 960, 216, 216, 119, 529, 601, 509,
                    176, 216, 844, 442, 983, 908, 940, 264, 934, 894, 216, 740, 216, 216, 216, 892,
                    115, 948, 216, 9, 115, 216, 720, 428, 216, 216, 432, 216, 216, 216, 352, 117,
                    458, 231, 482, 216, 216, 50, 886, 916, 306, 447, 38, 525, 742, 846, 216, 216,
                    216, 216, 604, 531, 754, 216, 592, 133, 864, 576, 484, 778, 558, 967, 216, 615,
                    634, 280, 216, 842, 216, 488, 492, 340, 586, 662, 843, 216, 634, 320, 964, 216,
                    107, 476, 566, 8, 545, 626, 694, 216, 940, 905, 874, 216, 216, 702, 216, 216,
                    628, 446, 416, 216, 216, 216, 970, 580, 216, 546, 970, 216, 19, 244, 38, 216,
                    514, 174, 984, 796, 874, 216, 415, 216, 96, 954, 398, 603, 216, 910, 670, 444,
                    808, 640, 216, 949, 311, 216, 954, 216, 216, 216, 216, 804, 216, 717, 595, 216,
                    728, 708, 216, 286, 756, 216, 216, 216, 502, 387, 216, 2, 984, 216, 398, 598,
                    184, 216, 944, 892, 882, 290, 216, 78, 446, 622, 216, 388, 118, 149, 216, 628,
                    216, 706, 15, 216, 216, 216, 336, 216, 216, 89, 784, 512, 268, 216, 582, 381,
                    762, 216, 216, 968, 653, 216, 216, 216, 264, 216, 660, 838, 822, 216, 216, 300,
                    388, 216, 220, 216, 216, 465, 889, 924, 614, 842, 216, 396, 216, 216, 216, 962,
                    747, 886, 298, 216, 216, 906, 306, 496, 216, 44, 458, 634, 430, 554, 370, 758,
                    532, 824, 216, 709, 216, 18, 216, 216, 610, 669, 662, 216, 366, 272, 640, 266,
                    534, 992, 842, 632, 810, 432, 520, 442, 891, 824, 332, 534, 216, 736, 676, 216,
                    640, 296, 45, 980, 754, 960, 498, 996, 776, 446, 362, 216, 479, 830, 716, 940,
                    216, 814, 680, 506, 69, 216, 680, 45, 440, 616, 158, 592, 554, 216, 216, 656,
                    216, 216, 216, 774, 143, 836, 216, 808, 216, 304, 894, 988, 216, 216, 842, 948,
                    79, 216, 798, 825, 958, 764, 216, 824, 954, 742, 634, 216, 130, 962, 368, 700,
                    216, 364, 714, 216, 836, 338, 900, 942, 648, 216, 216, 216, 948, 340, 856, 568,
                    594, 266, 656, 894, 598, 875, 254, 135, 216, 376, 216, 802, 694, 216, 884, 177,
                    216, 216, 982, 216, 300, 288, 478, 216, 216, 216, 636, 972, 216, 826, 310, 216,
                    216, 83, 740, 976, 261, 704, 216, 852, 850, 216, 357, 779, 716, 216, 974, 488,
                    883, 912, 670, 526, 678, 455, 216, 357, 216, 105, 466, 318, 794, 216, 300, 216,
                    678, 476, 372, 216, 952, 479, 216, 216, 718, 998, 216, 50, 288, 492, 25, 216,
                    216, 216, 216, 430, 481, 216, 216, 862, 752, 764, 216, 621, 216, 607, 216, 528,
                    590, 409, 216, 840, 99, 722, 216, 428, 216, 501, 844, 216, 690, 768, 216, 523,
                    216, 216, 192, 420, 692, 528, 925, 329, 964, 216, 828, 476, 986, 216, 556, 346,
                    216, 216, 820, 716, 216, 580, 216, 685, 516, 216, 664, 315, 216, 882, 216, 23,
                    930, 116, 220, 388, 788, 738, 216, 632, 360, 738, 593, 216, 759, 790, 216, 962,
                    789, 706, 398, 216, 392, 179, 810, 10, 646, 16, 922, 216, 216, 653, 216, 666,
                    928, 878, 726, 920, 216, 528, 266, 358, 216, 308, 288, 905, 225, 226, 216, 216,
                    968, 216, 932, 939, 29, 216, 59, 216, 416, 133, 414, 574, 502, 216, 756, 901,
                    372, 108, 52, 497, 630, 433, 864, 976, 472, 488, 216, 216, 418, 988, 296, 560,
                    345, 568, 464, 800, 840, 345, 25, 183, 360, 764, 216, 216, 216, 700, 702, 936,
                    326, 658, 216, 678, 682, 946, 216, 357, 630, 628, 660, 216, 216, 596, 853, 216,
                    628, 640, 916, 216, 986, 216, 216, 311, 288, 371, 810, 586, 832, 622, 216, 970,
                    746, 334, 13, 216, 712, 216, 216, 216, 216, 836, 274, 13, 484, 720, 216, 240,
                    75, 216, 216, 724, 630, 563, 472, 108, 287, 517, 216, 216, 216, 216, 388, 216,
                    548, 506, 744, 588, 990, 296, 216, 690, 216, 698, 216, 998, 488, 758, 356,
                    1000, 216, 678, 83, 926, 898, 920, 216, 438, 625, 598, 216, 690, 860, 216, 549,
                    216, 784, 268, 592, 690, 342, 878, 430, 976, 900, 216, 955, 216, 892, 459, 216,
                    240, 838, 571, 513, 732, 359, 216, 142, 216, 394, 216, 431, 216, 582, 216, 906,
                    641, 574, 710, 691, 282, 216, 29, 216, 395, 960, 856, 640, 360, 308, 216, 477,
                    740, 98, 216, 216, 404, 382, 216, 372, 242, 216, 617, 216, 216, 999, 53, 963,
                    618, 936, 742, 854, 510, 816, 216, 874, 216, 774, 903, 338, 958, 418, 906, 448,
                    586, 344, 216, 216, 710, 987, 521, 216, 216, 406, 886, 216, 216, 216, 216, 536,
                    207, 986, 415, 926, 465, 894, 216, 280, 886, 216, 481, 216, 643, 216, 528, 216,
                    736, 216, 974, 607, 580, 216, 216, 216, 860, 596, 779, 216, 216, 61, 393, 980,
                    216, 880, 358, 539, 984, 221, 930, 748, 429, 788, 320, 204, 890, 540, 338, 242,
                    852, 280, 216, 605, 892, 568, 412, 216, 216, 355, 912, 954, 216, 690, 420, 862,
                    216, 216, 704, 700, 216, 862, 656, 566, 216, 68, 603, 361, 846, 216, 392, 830,
                    716, 644, 216, 732, 612, 41, 216, 124, 216, 216, 216, 216, 554, 804, 216, 947,
                    922, 218, 82, 850, 216, 216, 788, 746, 972, 916, 283, 880, 627, 216, 216, 742,
                    760, 216, 306, 448, 874, 7, 216, 386, 707, 788, 764, 216, 626, 216, 366, 598,
                    216, 886, 988, 216, 216, 318, 502, 942, 488, 996, 150, 634, 362, 216, 600, 472,
                    296, 216, 774, 620, 216, 195, 306, 414, 216, 334, 802, 216, 740, 496, 508, 216,
                    880, 384, 362, 216, 906, 828, 644, 530, 195, 988, 396, 216, 408, 570, 646, 31,
                    216, 295, 216, 318, 216, 216, 826, 216, 529, 638, 540, 216, 216, 216, 216, 80,
                    308, 216, 236, 546, 212, 794, 471, 556, 898, 716, 216, 216, 295, 216, 760, 820,
                    724, 400, 216, 135, 305, 483, 216, 216, 256, 746, 906, 575, 670, 363, 372, 216,
                    606, 0, 912, 216, 964, 216, 306, 216, 216, 216, 216, 910, 418, 936, 216, 842,
                    436, 216, 610, 388, 864, 424, 728, 256, 950, 216, 940, 311, 700, 834, 565, 658,
                    216, 216, 454, 839, 604, 396, 216, 592, 216, 703, 508, 216, 63, 576, 216, 827,
                    356, 466, 599, 928, 216, 714, 664, 216, 864, 622, 216, 546, 380, 81, 742, 216,
                    428, 562, 216, 216, 837, 493, 364, 960, 430, 216, 546, 229, 216, 760, 105, 198,
                    395, 216, 866, 776, 948, 916, 168, 708, 216, 752, 216, 216, 480, 632, 550, 412,
                    662, 216, 822, 216, 586, 666, 16, 800, 597, 652, 216, 216, 368, 654, 650, 216,
                    466, 740, 896, 412, 876, 216, 216, 216, 216, 427, 626, 796, 690, 856, 216, 878,
                    67, 216, 216, 816, 299, 702, 141, 907, 216, 678, 774, 940, 838, 408, 216, 216,
                    698, 608, 296, 778, 394, 728, 825, 361, 379, 480, 216, 950, 216, 216, 570, 216,
                    348, 954, 644, 216, 386, 938, 216, 216, 703, 784, 866, 216, 306, 660, 770, 730,
                    981, 958, 722, 464, 990, 542, 216, 754, 990, 640, 216, 621, 550, 822, 485, 888,
                    703, 906, 98, 910, 528, 442, 216, 464, 861, 91, 980, 348, 273, 157, 216, 417,
                    862, 396, 882, 554, 560, 216, 296, 676, 216, 762, 532, 580, 216, 189, 634, 65,
                    412, 13, 594, 448, 631, 114, 253, 990, 780, 160, 216, 216, 216, 192, 876, 682,
                    560, 282, 321, 216, 273, 398, 420, 666, 216, 854, 599, 25, 797, 913, 216, 89,
                    791, 216, 216, 944, 340, 216, 761, 193, 960, 216, 684, 832, 883, 216, 448, 216,
                    216, 768, 664, 300, 892, 382, 86, 50, 448, 496, 482, 216, 216, 216, 478, 664,
                    597, 912, 216, 733, 659, 287, 216, 658, 216, 602, 424, 216, 590, 837, 799, 216,
                    135, 648, 940, 216, 216, 484, 564, 216, 748, 408, 776, 83, 240, 988, 558, 602,
                    216, 290, 684, 216, 939, 580, 170, 967, 546, 456, 566, 678, 874, 320, 914, 468,
                    1000, 216, 662, 463, 546, 484, 550, 820, 664, 844, 216, 550, 668, 532, 814,
                    216, 733, 586, 216, 216, 910, 288, 216, 432, 216, 216, 460, 847, 216, 344, 54,
                    936, 591, 622, 902, 228, 816, 216, 352, 216, 216, 403, 568, 216, 562, 216, 843,
                    796, 253, 216, 879, 216, 216, 216, 216, 458, 201, 1000, 216, 273, 216, 279,
                    216, 216, 5, 216, 802, 636, 216, 904, 216, 216, 662, 169, 524, 424, 842, 827,
                    216, 216, 804, 368, 216, 887, 314, 648, 113, 216, 808, 884, 428, 682, 216, 624,
                    216, 681, 570, 216, 684, 477, 447
                ]
            ),
            12687
        );
    }
}