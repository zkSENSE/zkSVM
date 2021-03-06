#![allow(non_snake_case)]
#[macro_use]
extern crate criterion;

use criterion::Criterion;
use num_bigint::BigInt;
use zkSENSE_rust_proof::zkSVM;

fn sensor_operations(c: &mut Criterion) {
    let label_proof = format!("Proving correctness of operations");
    let label_verify = format!("Verifying correctness of operations");

    let acc_x_pad_zeros: Vec<BigInt> = vec![100005003, 100064379, 99749000, 100026383, 100889452, 99948879, 99428519, 100492287, 100130966, 100030376, 99940704, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let acc_x_pad_zeros_sec_2: Vec<BigInt> = vec![99685881, 99886759, 99972156, 99851992, 99967340, 100296425, 100338349, 99996519, 99807624, 99983228, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let acc_y_pad_zeros: Vec<BigInt> = vec![99590298, 99572182, 99900177, 100244723, 99701807, 99075163, 99547737, 100060770, 100172506, 100027513, 99550404, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let acc_y_pad_zeros_sec_2: Vec<BigInt> = vec![99542136, 99807960, 100008313, 99993024, 99791641, 99599154, 99673585, 99770623, 99749219, 99806849, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let acc_z_pad_zeros: Vec<BigInt> = vec![99915568, 100084743, 100651309, 101476960, 100857768, 99091060, 99419434, 101114295, 102136665, 100873597, 99765532, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let acc_z_pad_zeros_sec_2: Vec<BigInt> = vec![99809079, 100320819, 100515097, 100504241, 100248353, 100024515, 100367264, 100592313, 100408090, 100270467, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_x_pad_zeros: Vec<BigInt> = vec![100166377, 100181516, 100190972, 100195747, 100195086, 100188924, 100182145, 100174502, 100143521, 100081511, 100019889, 99975503, 99944950, 99925605, 99890800, 99857400, 99826010, 99807657, 99825230, 99863139, 99912919, 99970110, 100026798, 100075845, 100122885, 100165890, 100200134, 100224296, 100247075, 100269365, 100299959, 100312969, 100294766, 100238941, 100157170, 100066256, 99980240, 99910267, 99861349, 99821687, 99793474, 99775558, 99770494, 99773111, 99780437, 99794180, 99805466, 99821552, 99837551, 99857665, 99873519, 99884136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_x_pad_zeros_sec_2: Vec<BigInt> = vec![99892405, 99890778, 99886229, 99878356, 99875703, 99878483, 99882100, 99889137, 99891324, 99896470, 99894754, 99899139, 99900550, 99899914, 99900080, 99901809, 99900094, 99898378, 99902019, 99902125, 99908241, 99914878, 99922639, 99933897, 99942754, 99952381, 99966504, 99978535, 99988153, 99994973, 99991189, 99981615, 99965386, 99950920, 99937894, 99927375, 99915602, 99908984, 99905905, 99901120, 99899405, 99899820, 99902241, 99907387, 99911922, 99914483, 99922198, 99925817, 99928574, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_y_pad_zeros: Vec<BigInt> = vec![100011467, 100015483, 100012801, 100011730, 100002176, 99990485, 99968027, 99937457, 99953606, 100010808, 100050752, 100072680, 100077039, 100079334, 100076994, 100061752, 100037605, 99992490, 99927895, 99869309, 99845362, 99839765, 99856893, 99873713, 99886364, 99888635, 99895826, 99913679, 99949462, 99971804, 99969910, 99943989, 99927903, 99926317, 99917317, 99924518, 99930193, 99923721, 99920228, 99922406, 99937762, 99969877, 99997030, 100002634, 99989172, 99960701, 99938486, 99934585, 99934817, 99963759, 99994599, 100029232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_y_pad_zeros_sec_2: Vec<BigInt> = vec![100054383, 100070218, 100083090, 100083613, 100084497, 100081351, 100069578, 100065796, 100066623, 100069225, 100072185, 100076086, 100081634, 100087499, 100096408, 100096755, 100093312, 100082826, 100074249, 100067663, 100066873, 100071144, 100076290, 100079040, 100083417, 100094473, 100109285, 100126414, 100129096, 100118349, 100103229, 100076394, 100051454, 100032022, 100019922, 100013125, 100016572, 100027351, 100039381, 100047064, 100046148, 100052031, 100061971, 100069483, 100074910, 100074910, 100076853, 100071668, 100068911, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_z_pad_zeros: Vec<BigInt> = vec![99991687, 99989327, 99988491, 99991550, 99996194, 100002673, 100006885, 100006732, 99997641, 99983503, 99973004, 99968929, 99981259, 100003406, 100031350, 100057889, 100084547, 100111693, 100134133, 100143168, 100139703, 100120701, 100093291, 100058984, 100025206, 99990222, 99960707, 99937458, 99920803, 99918957, 99944124, 99991272, 100033313, 100067218, 100079172, 100090492, 100096854, 100103066, 100108212, 100110795, 100110945, 100108467, 100107606, 100099029, 100086375, 100073354, 100065498, 100057426, 100055711, 100050240, 100046701, 100041555, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();
    let gyr_z_pad_zeros_sec_2: Vec<BigInt> = vec![100042477, 100038827, 100044608, 100049935, 100055134, 100053705, 100050277, 100049388, 100048617, 100048504, 100044132, 100039512, 100038198, 100032844, 100025208, 100021003, 100014681, 100007059, 100000238, 99995092, 99990526, 99987920, 99990621, 99993286, 99999098, 100008484, 100023711, 100041735, 100054680, 100063138, 100065879, 100068385, 100072083, 100076814, 100081263, 100082234, 100084035, 100083299, 100080604, 100076162, 100069184, 100062502, 100051528, 100047800, 100043931, 100042184, 100042568, 100043931, 100043994, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        .iter().map(|&x| BigInt::from(x as u64)).collect();

    let acc_pad_zeros: [Vec<BigInt>; 3] = [acc_x_pad_zeros.clone(), acc_y_pad_zeros.clone(), acc_z_pad_zeros.clone()];
    let acc_pad_zeros_sec_2: [Vec<BigInt>; 3] = [acc_x_pad_zeros_sec_2.clone(), acc_y_pad_zeros_sec_2.clone(), acc_z_pad_zeros_sec_2.clone()];
    let gyr_pad_zeros: [Vec<BigInt>; 3] = [gyr_x_pad_zeros.clone(), gyr_y_pad_zeros.clone(), gyr_z_pad_zeros.clone()];
    let gyr_pad_zeros_sec_2: [Vec<BigInt>; 3] = [gyr_x_pad_zeros_sec_2.clone(), gyr_y_pad_zeros_sec_2.clone(), gyr_z_pad_zeros_sec_2.clone()];

    // Once proven correctness, we will add the diff vectors
    let all_sensor_vectors: Vec<[Vec<BigInt>; 3]> = vec![
        acc_pad_zeros,
        acc_pad_zeros_sec_2,
        gyr_pad_zeros,
        gyr_pad_zeros_sec_2
    ];

    let size_vec_acc = 11;
    let size_vec_acc_sec_2 = 10;
    let size_vec_gyr = 52;
    let size_vec_gyr_sec_2 = 49;

    let size_sensors = vec![size_vec_acc, size_vec_acc_sec_2, size_vec_gyr, size_vec_gyr_sec_2];

    let zkSVM = zkSVM::create(&all_sensor_vectors, &size_sensors)
        .expect("Error generating the proof");

    c.bench_function(&label_proof, move |b| {
        b.iter(|| {
            zkSVM::create(&all_sensor_vectors, &size_sensors)
                .expect("Error generating the proof");
        })
    });

    c.bench_function(&label_verify, move |b| {
        b.iter(|| {
            zkSVM.clone().verify().unwrap();
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
    sensor_operations
);

criterion_main!(benches);