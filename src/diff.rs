
// On va lire petit à petit chaque ELEMENT de la partition. On va donc commencer par les mesures, puis ensuite leur contenu
// Effectue un LCS sur les mesures. On toppera donc comme modifiée la mesure entière, et ce sera au moment de la relecture
// de montrer en détail ce qui a été modifié ou ajouté ou supprimé.
pub fn LCSMeasuresLength(src: &super::types::Part, dst: &super::types::Part) -> Vec<Vec<u32>> {
    let i = src.measures.len() + 1;
    let j = dst.measures.len() + 1;
    let mut c = vec![vec![0; j]; i];
    if i == 0 || j == 0 {
        return c;
    }
    // Valeur de la plus longue division de la mesure
    let mut longestDivisionSrc = 0.;
    let mut longestDivisionDst = 0.;
    for k in 0..(i-1) {
        c[k][0] = 0;
    }
    for k in 0..(j-1) {
        c[0][k] = 0;
    }
    for k in 1..i {
        for l in 1..j {
            longestDivisionSrc = match &src.measures[k-1].attributes {
                None => longestDivisionSrc,
                Some(a) => a.divisions as f32
            };
            longestDivisionDst = match &dst.measures[l-1].attributes {
                None => longestDivisionDst,
                Some(a) => a.divisions as f32
            };
            let mut b = true;
            if src.measures[k-1].notes.len() == dst.measures[l-1].notes.len() {
                for ni in 0..src.measures[k-1].notes.len() {
                    // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
                    b = b && (src.measures[k-1].notes[ni].pitch == dst.measures[l-1].notes[ni].pitch)
                    && (src.measures[k-1].notes[ni].typee == dst.measures[l-1].notes[ni].typee)
                    && (src.measures[k-1].notes[ni].duration as f32 / longestDivisionSrc == dst.measures[l-1].notes[ni].duration as f32 / longestDivisionDst)
                };
                if b {
                    c[k][l] = c[k-1][l-1] + 1
                }
            }
            else {
                c[k][l] = std::cmp::max(c[k][l-1], c[k-1][l])
            }
        }
    }
    return c;
}

pub fn print_diff(src: &super::types::ScorePartwise, dst: &super::types::ScorePartwise, lcs : Vec<Vec<u32>>, i: isize, j: isize) {
    if i > 0 && j > 0 && src.parts[0].measures[(i-1) as usize].notes.len() == dst.parts[0].measures[(j-1) as usize].notes.len() {
        let mut b = true;
        for ni in 0..src.parts[0].measures[(i-1) as usize].notes.len() {
            // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
            b = b && (src.parts[0].measures[(i-1) as usize].notes[ni].pitch == dst.parts[0].measures[(j-1) as usize].notes[ni].pitch)
            && (src.parts[0].measures[(i-1) as usize].notes[ni].typee == dst.parts[0].measures[(j-1) as usize].notes[ni].typee);
        }
        if b {
            print_diff(src, dst, lcs, i-1, j-1);
            println!(" {:#?}", src.parts[0].measures[(i-1) as usize]);
        };
    }
    else if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] >= lcs[(i-1) as usize][j as usize]) {
        print_diff(src, dst, lcs, i, j-1);
        println!("+ {:#?}", dst.parts[0].measures[(j-1) as usize]);
    }
    else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] < lcs[(i-1) as usize][j as usize]) {
        print_diff(src, dst, lcs, i-1, j);
        println!("- {:#?}", src.parts[0].measures[(i-1) as usize]);
    }
    else {
        println!("");
    }
}

pub fn diff(src: &super::types::ScorePartwise, dst: &super::types::ScorePartwise) -> Vec<super::types::Element> {
    let res = Vec::new();
    let lcs = LCSMeasuresLength(&src.parts[0], &dst.parts[0]);
    let i : isize = (lcs.len() as isize) - 1;
    let j : isize = (lcs[0].len() as isize) - 1;
    // TODO : Calculer le diff sur chacune des parts (portées)
    // Montre les ajouts par un +, les unmodified par un rien, et les suppressions par un -
    // Pour les modifications, récup la mesure qui a été ajoutée et celle supprimée juste avant / après, et regarder dedans en profondeur
    print_diff(src, dst, lcs, i, j);
    // println!("{:#?}", lcs);
    return res;
    
}