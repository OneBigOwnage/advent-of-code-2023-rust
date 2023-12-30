use regex::Regex;
use std::{fmt::Display, ops::Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Velocity3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Hailstone {
    starting_position: Point3D,
    velocities: Velocity3D,
}

impl Hailstone {
    fn position_after(&self, microseconds: usize) -> Point2D {
        Point2D {
            x: self.starting_position.x as f64 + self.velocities.x as f64 * microseconds as f64,
            y: self.starting_position.y as f64 + self.velocities.y as f64 * microseconds as f64,
        }
    }

    fn start(&self) -> Point2D {
        Point2D {
            x: self.starting_position.x as f64,
            y: self.starting_position.y as f64,
        }
    }

    fn intersects_at(&self, other: &Self) -> Option<Point2D> {
        let determinant =
            self.velocities.x * other.velocities.y - self.velocities.y * other.velocities.x;

        if determinant == 0.0 {
            return None; //particles don't meet
        }

        let foo_self = self.velocities.x * self.starting_position.y
            - self.velocities.y * self.starting_position.x;

        let foo_other = other.velocities.x * other.starting_position.y
            - other.velocities.y * other.starting_position.x;

        return Some(Point2D {
            x: (other.velocities.x * foo_self - self.velocities.x * foo_other) / determinant,
            y: (other.velocities.y * foo_self - self.velocities.y * foo_other) / determinant,
        });
    }

    fn is_in_future(&self, point: Point2D) -> bool {
        let is_x_in_future = match self.velocities.x.is_sign_positive() {
            true => point.x >= self.starting_position.x,
            false => point.x <= self.starting_position.x,
        };

        let is_y_in_future = match self.velocities.y.is_sign_positive() {
            true => point.y >= self.starting_position.y,
            false => point.y <= self.starting_position.y,
        };

        is_x_in_future && is_y_in_future
    }

    fn is_point_on_trajectory(&self, point: Point2D) -> bool {
        let deviation_from_line = (point.x - self.starting_position.x) * self.velocities.y
            - (point.y - self.starting_position.y) * self.velocities.x;

        deviation_from_line.abs() < 0.0001
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.starting_position.x,
            self.starting_position.y,
            self.starting_position.z,
            self.velocities.x,
            self.velocities.y,
            self.velocities.z
        )
    }
}

fn main() {
    assert_eq!(2, part1(&test_input(), 7.0, 27.0));
    assert_eq!(
        19_976,
        part1(&input(), 200_000_000_000_000.0, 400_000_000_000_000.0)
    );
    assert_eq!(47, part2(&test_input()));
    assert_eq!(849_377_770_236_905, part2(&input()));
}

fn part1(input: &str, lower_bound: f64, upper_bound: f64) -> usize {
    let pairs = make_pairs(&parse(input));

    pairs
        .iter()
        .filter(|(a, b)| {
            #[cfg(debug_assertions)]
            println!(
                "Hailstone A: {a}\nHailstone B: {b}\nIntersection point: {:?}\n",
                a.intersects_at(b).map(|point| (point.x, point.y))
            );

            match a.intersects_at(b) {
                Some(intersection) => {
                    (lower_bound..=upper_bound).contains(&intersection.x)
                        && (lower_bound..=upper_bound).contains(&intersection.y)
                        && a.is_in_future(intersection)
                        && b.is_in_future(intersection)
                }
                None => false,
            }
        })
        .count()
}

fn part2(input: &str) -> usize {
    let hailstones = parse(input);

    let mut x: Option<i32> = None;
    let mut y: Option<i32> = None;
    let mut z: Option<i32> = None;

    let translate = |hailstone: &Hailstone, by: Point2D| Hailstone {
        starting_position: hailstone.starting_position,
        velocities: Velocity3D {
            x: hailstone.velocities.x - by.x,
            y: hailstone.velocities.y - by.y,
            z: hailstone.velocities.z,
        },
    };

    let bruteforce = 250;

    'bruteforce: for i in -bruteforce..=bruteforce {
        for ii in -bruteforce..=bruteforce {
            let potential = Point2D {
                x: i as f64,
                y: ii as f64,
            };

            for iii in 0..hailstones.len() {
                for iiii in 0..hailstones.len() {
                    if let Some(intersection) = translate(&hailstones[iii], potential)
                        .intersects_at(&translate(&hailstones[iiii], potential))
                    {
                        if hailstones.iter().all(|hailstone| {
                            translate(hailstone, potential).is_point_on_trajectory(intersection)
                        }) {
                            x = Some(potential.x as i32);
                            y = Some(potential.y as i32);

                            break 'bruteforce;
                        }
                    }
                }
            }
        }
    }

    let fake_hailstones: Vec<Hailstone> = hailstones
        .iter()
        .map(|hailstone| Hailstone {
            starting_position: Point3D {
                x: hailstone.starting_position.z,
                y: hailstone.starting_position.y,
                z: hailstone.starting_position.x,
            },
            velocities: Velocity3D {
                x: hailstone.velocities.z,
                y: hailstone.velocities.y,
                z: hailstone.velocities.x,
            },
        })
        .collect();

    'bruteforce: for i in -bruteforce..=bruteforce {
        for ii in -bruteforce..=bruteforce {
            let potential = Point2D {
                x: i as f64,
                y: ii as f64,
            };

            for iii in 0..hailstones.len() {
                for iiii in 0..hailstones.len() {
                    if let Some(intersection) = translate(&fake_hailstones[iii], potential)
                        .intersects_at(&translate(&fake_hailstones[iiii], potential))
                    {
                        if fake_hailstones.iter().all(|hailstone| {
                            translate(hailstone, potential).is_point_on_trajectory(intersection)
                        }) {
                            z = Some(potential.x as i32);

                            break 'bruteforce;
                        }
                    }
                }
            }
        }
    }

    let first = hailstones.first().unwrap();
    let second = hailstones.last().unwrap();

    let a = x.unwrap() as i64 - first.velocities.x as i64;
    let b = x.unwrap() as i64 - second.velocities.x as i64;
    let c = y.unwrap() as i64 - first.velocities.y as i64;
    let d = y.unwrap() as i64 - second.velocities.y as i64;
    let foo = (b * (second.starting_position.y - first.starting_position.y) as i64
        - (second.starting_position.x - first.starting_position.x) as i64 * d)
        / (a * d - b * c);

    let stone_vel = Velocity3D {
        x: x.unwrap() as f64,
        y: y.unwrap() as f64,
        z: z.unwrap() as f64,
    };

    let stone_origin = Point3D {
        x: first.starting_position.x + (first.velocities.x - stone_vel.x) * foo as f64,
        y: first.starting_position.y + (first.velocities.y - stone_vel.y) * foo as f64,
        z: first.starting_position.z + (first.velocities.z - stone_vel.z) * foo as f64,
    };

    let magic_stone = Hailstone {
        starting_position: stone_origin,
        velocities: stone_vel,
    };

    println!("Magic stone: {:?}", magic_stone);

    stone_origin.x as usize + stone_origin.y as usize + stone_origin.z as usize
}

fn make_pairs(hailstones: &Vec<Hailstone>) -> Vec<(Hailstone, Hailstone)> {
    let mut pairs = vec![];

    for i in 0..hailstones.len() {
        for ii in i + 1..hailstones.len() {
            pairs.push((hailstones[i], hailstones[ii]));
        }
    }

    pairs
}

fn parse(input: &str) -> Vec<Hailstone> {
    let re =
        Regex::new(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+([-]?\d+),\s+([-]?\d+),\s+([-]?\d+)").unwrap();

    input
        .split("\n")
        .map(|line| {
            let (_, [x, y, z, vel_x, vel_y, vel_z]) = re.captures(line).unwrap().extract();

            Hailstone {
                starting_position: Point3D {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                    z: z.parse().unwrap(),
                },
                velocities: Velocity3D {
                    x: vel_x.parse::<f64>().unwrap(),
                    y: vel_y.parse::<f64>().unwrap(),
                    z: vel_z.parse::<f64>().unwrap(),
                },
            }
        })
        .collect()
}

#[allow(dead_code)]
fn test_input() -> String {
    "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
        .to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "237822270988608, 164539183264530, 381578606559948 @ 115, 346, -342
287838354624648, 284335343503076, 181128681512377 @ -5, -84, 175
341046208911993, 120694764237967, 376069872241870 @ -74, 129, -78
275834119712623, 395388307575057, 177270820376760 @ 90, -111, -10
284284433233698, 358506322947508, 169341917878543 @ 20, 133, 71
236676388618224, 139432657314826, 343396041364471 @ 47, 28, 19
271686440086412, 267306686527596, 183118696313003 @ 53, 273, 93
286338526846979, 357920256353161, 178006715671210 @ 8, 227, -38
277084490601628, 287132189587528, 259351816392501 @ 15, -16, 11
271571599499731, 376770270435084, 104803601292067 @ 102, -12, 505
318770864097708, 243720191032512, 285990120629205 @ -143, 505, -396
295264015466268, 171618421304212, 325317744985951 @ -15, 75, -21
295209980801646, 191803613346922, 309943723274129 @ -14, 20, 20
198214675546904, 272335936655574, 258298762033323 @ 111, -68, 75
288187702040868, 388917296420926, 171504207459427 @ -10, 94, -111
184273974013684, 12373190633322, 278042492719100 @ 207, 570, -48
242275200795523, 225852639315987, 440385087222770 @ 42, -57, -84
243949320421141, 268304032408684, 341451505286299 @ 66, -16, -95
337226374533810, 195159327254164, 299179995694807 @ -66, 19, 31
280978859731038, 215170295891107, 178692577877350 @ 13, 269, 138
337911809851926, 261853551304374, 399832774762439 @ -62, -78, -69
271876383595711, 335519858346274, 217529147144208 @ 42, -22, 14
244568061493968, 266873230762202, 239488175392715 @ 253, 621, -325
278218955785750, 407933581871350, 197938715346263 @ 41, -226, -19
314577860549553, 285709232531452, 286894806222145 @ -34, -112, 67
346903057043322, 418398080440144, 364319795293105 @ -146, -263, -298
529734307887258, 491687159914888, 436016326126261 @ -260, -333, -86
298036599924295, 248170224604671, 234466977373475 @ -24, 57, 57
295755146155944, 353246595858338, 209282418695331 @ -29, -69, 33
266600460292728, 403513334509342, 320849937647635 @ 31, -234, -75
231006679712922, 393081183652762, 251917334976715 @ 138, -200, -43
277404865614638, 177724646431182, 259087150561410 @ 17, 246, -16
294273485384420, 347723573228264, 193340426330173 @ -27, -27, 64
257420279878103, 263328355308992, 248062342477880 @ 32, -68, 96
434900672112058, 290298764569852, 301751505292905 @ -246, -49, -34
301671581437108, 305053229519674, 321502352111145 @ -25, -94, -33
429942222949264, 506190509889614, 321583030567971 @ -258, -416, -91
285641539350422, 397645556746126, 157728603096529 @ 26, -34, 88
354509827387988, 454131837716572, 372132543279535 @ -73, -292, -11
261627860229688, 262242055885842, 336647314239233 @ 35, -20, -70
320979101845192, 190805827444153, 221082401500335 @ -69, 179, 77
211706423968692, 350415455618041, 128647701783597 @ 79, -181, 239
275452568927752, 467720794785324, 134459480144873 @ 59, -533, 290
291459698467848, 373962596611742, 167538716401475 @ -35, 73, 60
299657202351448, 313114237596262, 319960308993995 @ -43, 71, -334
308630014995110, 315446681057454, 220999351349073 @ -63, 22, 15
285892456499118, 379677540710299, 164413655043039 @ 25, 263, -27
276209258971816, 189819125818148, 448036230015819 @ 11, 57, -195
210221830156354, 222958574958400, 209061241541121 @ 158, 149, 90
254119705865920, 297912302891790, 246767788341103 @ 63, -18, 19
405284877472644, 379532656383042, 268898089141147 @ -128, -215, 91
303895961751448, 31584229395362, 265807548981035 @ -25, 225, 71
366525943480573, 198883057545712, 296676081696670 @ -166, 188, -86
223251479251376, 334265571617594, 166325665825519 @ 137, -77, 178
212721217954316, 272326131169380, 314543481692175 @ 81, -89, 27
312701197317523, 317117441367962, 361955412428795 @ -60, -38, -255
405625102662498, 248836196390791, 366885123318877 @ -155, -41, -61
314318949267930, 255539974944019, 245192149265146 @ -59, 71, 20
288341647642664, 265487544378174, 254278550738755 @ -6, 43, 7
325169426281992, 491762438822366, 447235590214147 @ -41, -326, -71
314432431821313, 276597311939416, 238523350747753 @ -60, 32, 31
340685445118664, 252886691872102, 421969081241747 @ -109, 65, -321
518724873304278, 456564296629681, 451304897002021 @ -275, -301, -137
318428281973807, 298369789433741, 282353139939853 @ -64, -29, -41
276558285764498, 310529373406700, 255522069749617 @ 20, -23, -20
313808524188693, 122860079058487, 293356898354965 @ -52, 275, -45
240565198143708, 475312732738102, 299458792958395 @ 42, -312, 67
300394602525054, 306224291503411, 184760969239898 @ -47, 107, 96
250941744603526, 304929793414300, 287727245107337 @ 78, -7, -96
279081475632633, 364505249947792, 170308625883640 @ 72, 176, 26
296672387625238, 214186656143587, 299648981265484 @ -15, -25, 46
320669983020712, 180234091496526, 328195337004451 @ -39, -9, 31
220381054242696, 143123197129054, 335086612129027 @ 97, 158, -65
235239270701064, 244927547234302, 287390155087075 @ 67, -20, 27
300624657978087, 268815888127186, 364016855273943 @ -38, 122, -339
254284503382152, 349776788114350, 253967081013859 @ 123, -9, -185
312745897552648, 283498721596862, 146292901159395 @ -81, 145, 224
348473921800452, 97797166792498, 374660366178763 @ -86, 171, -85
303878701802568, 116822912295262, 235780699050755 @ -44, 470, 5
377914742743812, 499692624253094, 502220244386187 @ -113, -354, -207
267636092044648, 228101696049862, 349599529626395 @ 15, -67, 18
63288015101003, 23982354461092, 124158946546965 @ 234, 164, 243
276553934698750, 365742070395452, 170580121096686 @ 93, 160, 26
429393880745468, 297685755958480, 356875862533173 @ -145, -136, 11
158248367090592, 289961901228707, 168385487273098 @ 211, -45, 184
306593194601382, 96203990160910, 520558009827286 @ -23, 54, -140
329177967436174, 559641347342922, 323946987430094 @ -67, -470, -46
264846633797803, 278109062996717, 300829492700720 @ 32, -33, -28
250292688268956, 235850198865862, 208447560484753 @ 101, 250, 48
270989988823668, 396465069265903, 196725204958195 @ 135, -113, -183
290118907362588, 382164936694672, 238326767193334 @ -25, 20, -570
254294894398350, 276805413032476, 221593371565573 @ 29, -112, 143
407887416560501, 246392181834867, 352230451862287 @ -196, 15, -107
248596989497808, 260892108790357, 128966334391975 @ 67, 29, 253
251205448433883, 367495507763212, 211157090303125 @ 132, -80, -17
314516290411408, 250883938752295, 266636882811808 @ -45, -7, 40
275723354776164, 443006917511755, 160236055271245 @ 47, -379, 167
196749291727940, 144396713220016, 365316166606183 @ 111, 92, -59
251657829325134, 181201466332948, 399894989981749 @ 32, -13, -40
445899774487058, 328598376657752, 363976412933705 @ -198, -147, -46
293372101198276, 461053426566090, 182960096633892 @ -57, -702, -106
225768545205384, 243230263526062, 278545680426979 @ 59, -75, 83
280515160876260, 269627035295554, 269419713803797 @ 9, 24, -14
190243295506058, 387222792161961, 203595141185873 @ 105, -221, 155
263234475302775, 331850722373651, 314435388680613 @ 22, -161, 35
314011313284048, 199642785582752, 274217218520515 @ -45, 76, 25
331614288807734, 473051562535336, 94423572966153 @ -64, -331, 290
433935552996948, 430484062461538, 337539393516903 @ -155, -268, 23
331592456844898, 305932229712737, 171646611368770 @ -79, -69, 178
307344419882425, 333201937160464, 302909338552321 @ -118, 212, -677
270518926153101, 86449098952513, 511012094230360 @ 22, 259, -348
240020699196477, 424040550640206, 425097807040036 @ 64, -266, -182
295338048514050, 95238093887650, 329261460654625 @ -16, 216, -48
188618248982418, 122641719185152, 288965092345513 @ 130, 145, 26
309857466476792, 362283419678917, 257681707495563 @ -181, 155, -655
341245479563934, 154817961450313, 385952016685555 @ -59, 11, -23
371075374168986, 137127765264840, 526417801749792 @ -107, 88, -246
214413812291096, 231375489913078, 303373889982883 @ 69, -67, 61
284656399883228, 404042925380886, 99221261967079 @ 30, -152, 760
281184965895998, 352539629000720, 226032461756881 @ 20, -23, -74
277398203601934, 394309271830587, 231362500064056 @ 9, -225, 106
298222167632110, 362367034372804, 130200999428421 @ -114, 281, 410
323354811736998, 206817667181262, 219968131740020 @ -107, 342, 12
267083473271757, 380667027663476, 203933367229771 @ 58, -152, 49
270103978824828, 273540500008277, 237258895954495 @ 31, 33, 37
225540438323656, 138452204955616, 467863837997979 @ 59, 32, -111
293151441060798, 364426209206692, 185955724889650 @ -35, 24, 6
300683754322959, 376836347331176, 262520127323023 @ -74, -56, -395
318897402546745, 338646575147035, 396273095321485 @ -84, -61, -413
300677720419399, 397381252905781, 152684686889890 @ -52, -193, 202
315138020341538, 122015961159720, 232045995004173 @ -40, 123, 110
344047921625785, 260263724116460, 344062078317558 @ -72, -68, -16
289022412599592, 251328485077006, 298962379773283 @ -7, 23, -37
336409606403432, 351406634416622, 193831470667139 @ -151, -65, 81
343141413811628, 107277307877918, 511576187255391 @ -60, 54, -144
230490532505840, 245533756664858, 142574745281649 @ 119, 112, 230
288785352215176, 81503241972062, 163738978403843 @ -6, 100, 201
330144449525215, 300260942328803, 216351646840374 @ -118, 51, 36
384218178916488, 340065156536158, 328696813600771 @ -165, -129, -82
270474320707410, 326501676600826, 238732214312971 @ 42, -16, -27
228806610982428, 160267739895652, 191780975717995 @ 77, 101, 157
317594597392248, 225818694140622, 278853235293555 @ -50, 33, 20
278395242271878, 74259435604178, 196432119542763 @ 10, 288, 141
298146454011807, 395630540923801, 197895863508403 @ -102, -87, -247
267945420411804, 197747019445186, 351133256213443 @ 17, -12, -7
456132992257280, 473868862585238, 422194863413355 @ -168, -309, -48
308027652560027, 214187338701511, 422422142473282 @ -76, 449, -746
271111618737837, 282810234074113, 130092414674043 @ 118, 707, 353
476723600318714, 265210473807048, 561890376345381 @ -207, -92, -225
220004414114151, 474711657927613, 257931279270823 @ 62, -311, 109
293906351933832, 404040027608998, 163946635144267 @ -101, -109, -26
273272209573232, 202783734343574, 364998314862147 @ 9, -48, 9
276225358419656, 375050641811998, 103666416250371 @ 86, 47, 569
288736224506783, 82536282031182, 159361455541015 @ -6, 118, 205
272195371998984, 123557220082078, 466335101539195 @ 11, 46, -108
251041052789992, 228965889483582, 314174875211347 @ 39, -30, 19
295331056328776, 313205807220658, 226792090992231 @ -37, 176, -117
290213154549928, 295399802355502, 164735656447299 @ -15, 245, 149
243728516252385, 295326162997494, 273997341739280 @ 46, -114, 72
270522281097804, 426680456948414, 250923451492419 @ 106, -338, -439
326076048220046, 375231430650591, 536131208800645 @ -79, -177, -533
289766561484600, 243270194981086, 241462314043083 @ -11, 278, -72
254453310003759, 263147711255437, 194812463353549 @ 34, -75, 163
363190835759360, 259380765909487, 341063463332105 @ -189, 125, -253
295394269953376, 389997719177182, 129553761886827 @ -84, -10, 416
279986879957288, 253567462909454, 135239510020883 @ 5, -47, 234
297449919710331, 203245524060142, 326837860362607 @ -16, -10, 13
279203966678288, 36179106452062, 167425681565635 @ 6, 230, 193
223889308617672, 144166564658791, 243511382122000 @ 59, 19, 122
351335735721732, 248436456723508, 239022680942151 @ -71, -79, 123
287838354624648, 301518234147037, 249620659444303 @ -5, -25, 13
319082495840320, 230458210192596, 278534240647849 @ -49, 7, 34
354959692134585, 353546157412435, 234996095129107 @ -92, -173, 105
350032659948616, 366306669663886, 189402029824249 @ -69, -202, 175
278270137151328, 321184494708202, 173377398590815 @ 17, -39, 160
279747570244464, 406976251103317, 183432554229142 @ 83, -185, -162
290204005905312, 384761720895616, 108370945184107 @ -17, -107, 421
243663581226102, 390865011957589, 320027208954514 @ 121, -187, -272
220565234418476, 169901125653918, 181681051895528 @ 119, 198, 156
271294584181344, 356958098694910, 168312281714719 @ 99, 102, 93
394762416075693, 402959495717512, 551301005411200 @ -112, -240, -186
219738791315879, 140554145421786, 52718699676060 @ 98, 162, 362
375396790076256, 189081398922637, 373452998910661 @ -93, -25, -9
332313113192520, 384153169748158, 193061775523267 @ -127, -171, 95
314007487042874, 346644145837748, 183020085926745 @ -138, 87, 42
304382206361178, 428838878801212, 175539252234775 @ -156, -395, -29
331475788827544, 249312020396002, 253830240621203 @ -73, 5, 53
295987102632663, 370397825726117, 177560343206115 @ -50, -15, 57
270848845808280, 105017551360192, 247692485050875 @ 19, 185, 77
292120639945590, 395109543152937, 487601271529970 @ -11, -225, -258
345556591185648, 148025692841872, 328667915025715 @ -105, 209, -95
300070967457480, 405088576121470, 190818656497315 @ -173, -142, -363
311173749288338, 419707381322667, 354306943595820 @ -43, -261, -118
276912089856930, 330821262369184, 185048821854409 @ 84, 419, -80
485453774879898, 295964069865754, 503504311733824 @ -255, -102, -232
329112863378258, 343911402196278, 303554036184858 @ -115, -66, -196
181107261039323, 181824375306572, 240596580391020 @ 140, 64, 92
291056468446904, 220670097777066, 61697943754483 @ -9, -11, 325
200302589597028, 214794512989942, 464100543648355 @ 86, -44, -111
264613058016948, 361751176589887, 323832436813495 @ 39, -153, -115
288948355456798, 386645378934907, 167568031185160 @ -15, -13, 46
260264404209480, 273359274680074, 351944901267259 @ 27, -88, -19
282547970794116, 359128493598406, 197350554871603 @ 18, -16, 5
262946756913048, 130017904668342, 220772650319035 @ 25, 91, 130
307542446946718, 278055151221152, 356995685746520 @ -31, -72, -58
263424531390033, 225952901540662, 312731019979615 @ 42, 110, -99
328087098783918, 399998779846982, 344279186833225 @ -92, -222, -206
304322006866158, 401963297162522, 181843508319845 @ -119, -174, -13
280144662617238, 209838929661238, 300922118702113 @ 5, 14, 19
292282685224800, 199624529849566, 286361265560107 @ -11, 38, 31
253443711623816, 256976692025182, 328558986145795 @ 38, -55, -8
279853117517878, 441128878162179, 135240263505508 @ 65, -497, 334
275773768913799, 382301829553624, 190867411924046 @ 106, 32, -172
271029327974932, 336111133972446, 194811840242114 @ 23, -122, 140
309925010104224, 252210628222170, 422829655826797 @ -41, 12, -230
301021335251088, 368862652339546, 184474019771815 @ -95, 51, -28
316629360285746, 215703381148914, 346448609051877 @ -52, 72, -106
346432183195971, 199432514880007, 398852183056639 @ -64, -35, -35
303204744896268, 354560115624706, 199296104851741 @ -65, -22, 19
227696467178984, 83142048148382, 257230426534659 @ 119, 430, -7
268791503428410, 224600203494064, 293201998260165 @ 22, 17, 12
314535683595246, 344725891287556, 291961561427125 @ -59, -112, -73
303265318110168, 49647408572866, 113876349112975 @ -25, 221, 262
209889530006344, 179105924801950, 268451718962787 @ 101, 68, 54
278015688720168, 282802075422502, 257864175142735 @ 11, -38, 39
282595386388802, 172729126759918, 139054347435675 @ 6, 254, 237
240365011863648, 334438793321362, 207015746507395 @ 73, -121, 121
360759055204432, 517922923281793, 372347715105458 @ -253, -607, -544
348826423111264, 352627243958144, 327573739687855 @ -97, -159, -53
277423022849548, 372481395187902, 184445516352525 @ 65, 22, -24
301279715445598, 250580759939786, 348056811538293 @ -30, 52, -154
280879231546920, 370762714998136, 200128022249818 @ 27, -56, -20
163489181111538, 155645262695119, 375285125590834 @ 125, 19, -20
374130299256992, 285945240712074, 285924900666027 @ -147, -41, -9
357075110197091, 163906755478373, 388332200430123 @ -72, -9, -15
433343529841049, 349558027711957, 398541965252449 @ -246, -145, -197
305899038888419, 370726010096496, 234057297414931 @ -76, -84, -119
274465878157970, 161740876461411, 302370496322192 @ 9, 13, 56
253002620293152, 297800943505528, 185841899504356 @ 67, -12, 140
276486738079177, 336536638497106, 219188534494300 @ 36, 26, -38
288513111363700, 217947701070834, 276790797375275 @ -6, 39, 27
265461303571669, 144427786021114, 178747390113758 @ 18, 26, 186
293029676171958, 499474424445362, 393271305885273 @ -20, -500, -490
291763471509486, 403139606444767, 233286907628215 @ -43, -155, -603
499566284795684, 335288005057006, 308538684726527 @ -234, -166, 44
177015841424412, 309258819054302, 273019312478467 @ 166, -90, 25
287838354624648, 243731737998892, 215895010071037 @ -5, 195, 40
296331018641872, 425467903524350, 145793377476045 @ -81, -362, 242
239179639831152, 258752246142262, 248303115581503 @ 137, 200, -74
264288681383232, 384917073369462, 299417067001607 @ 37, -200, -52
270252780911188, 210718300952072, 358734622831242 @ 15, -20, -22
260399027782968, 273011831764294, 401028823898851 @ 35, -46, -151
325304679003464, 205785663856350, 272005497262083 @ -73, 126, -7
214880557175304, 323275105958710, 187599172535947 @ 139, -73, 140
420588222489144, 202167135493642, 463419473850247 @ -153, -15, -134
272738011490050, 249146502479375, 288429325114933 @ 41, 249, -209
371124598446776, 281636051035470, 368299193909475 @ -168, 7, -213
305347219259190, 267045193369012, 132050867022772 @ -59, 200, 268
298975771877694, 254730359903272, 181115346710323 @ -68, 645, 33
288729689382508, 384429686405342, 160976570089649 @ -15, 70, 78
233930348914236, 377013728183254, 218878828322893 @ 97, -182, 84
210698018986590, 180529569201934, 300864043150003 @ 72, -18, 65
240045138394912, 165182303992178, 296997530510891 @ 71, 144, -19
294697865920929, 160130410879648, 386710051954864 @ -12, 8, -26
278858075268880, 488643324655478, 159812366150105 @ 51, -722, 147
61133119961048, 141891791993862, 108543972789270 @ 363, 190, 282
325214939349048, 471633636674170, 199792938027823 @ -105, -407, 80
292449432480426, 188546609675666, 302289612147995 @ -11, 42, 17
290589896226980, 403748966400786, 205989867624735 @ -34, -153, -388
352376347009040, 235129632406250, 281095873659519 @ -99, 9, 24
329011539818048, 341262617378942, 348155867539875 @ -90, -102, -195
289918346674948, 244473042582562, 493865697305195 @ -7, -88, -115
317785420889448, 179365281791422, 156129241051715 @ -140, 803, 183
301479827139906, 194688838412734, 328957597990473 @ -23, 38, -21
324502989170948, 150510804009662, 189111086717095 @ -81, 294, 133
175507392045286, 161232650395080, 371609074047267 @ 132, 57, -55
231946611688983, 6232484678869, 70338576845464 @ 60, 223, 308
305612247210783, 195715933715212, 223632341207965 @ -32, 80, 103
369074289172168, 366241736292038, 496886086327887 @ -85, -204, -126
395564292520218, 357199500974067, 386718037566655 @ -119, -191, -35
72109498749648, 142830369719362, 263826790392895 @ 229, 43, 92
277147873660958, 316369432709042, 235216776488745 @ 24, 12, -18
273513892517632, 467667147360710, 69215407247667 @ 56, -483, 555
325684213667262, 98471331850379, 315289015119290 @ -47, 99, 32
217657436845928, 166463321580382, 299219230724355 @ 77, 38, 41
255826889638308, 276370148048478, 208341751596563 @ 55, 6, 105
157603401177453, 217338452515292, 250578245558620 @ 244, 124, 22
310125052053632, 225820990934186, 248492901981875 @ -63, 237, -43
286436209380206, 395796739195454, 164093093776283 @ 12, -42, 29
282413222554614, 339933053498026, 160610282624119 @ 32, 248, 135
304815638486933, 460888258973902, 151183542722568 @ -90, -490, 204
230142835284552, 94971411384742, 102692853843955 @ 83, 235, 287
373690328871702, 241853041530691, 243507861721504 @ -147, 33, 60
230066103376216, 214147887007846, 253086824360647 @ 63, -16, 94
295304586941616, 368155004755054, 225473324219491 @ -99, 314, -751
310195329842576, 301597464398336, 193730909222365 @ -57, 9, 112
307922521992649, 294273227845947, 193236942756984 @ -126, 465, -52
210349946247572, 195780938336899, 360660555671603 @ 71, -37, 9
297619904476788, 337872394386382, 177402934087375 @ -42, 34, 108
265204087593818, 380617731469962, 506855129386247 @ 30, -200, -337"
        .to_string()
}
