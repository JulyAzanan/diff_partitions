#[derive(Debug)]
pub enum Diff {
    Unmodified(super::types::Element),
    Added(super::types::Element),
    Removed(super::types::Element),
    Modified(super::types::Element, Box<Vec<Diff>>),
}

pub fn LCSNotesLength(src: &super::types::Mesure, dst: &super::types::Mesure) -> Vec<Vec<u32>> {
    let i = src.notes.len() + 1;
    let j = dst.notes.len() + 1;
    let mut c = vec![vec![0; j]; i];
    if i == 0 || j == 0 {
        return c;
    }
    for k in 0..(i-1) {
        c[k][0] = 0;
    }
    for k in 0..(j-1) {
        c[0][k] = 0;
    }
    for k in 1..i {
        for l in 1..j {
            if (src.notes[k-1].typee == dst.notes[l-1].typee) && (src.notes[k-1].pitch == dst.notes[l-1].pitch)
            {
                c[k][l] = c[k-1][l-1] + 1
            }
            else {
                c[k][l] = std::cmp::max(c[k][l-1], c[k-1][l])
            }
        }
    }
    return c;
}

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

pub fn compute_diff_notes(src: &super::types::Mesure, dst: &super::types::Mesure, lcs: Vec<Vec<u32>>, i: isize, j: isize) -> Vec<Diff> {
    if i > 0 && j > 0 && (src.notes[(i-1) as usize].pitch == dst.notes[(j-1) as usize].pitch)
    && (src.notes[(i-1) as usize].typee == dst.notes[(j-1) as usize].typee) {
        let mut res = compute_diff_notes(src, dst, lcs, i-1, j-1);
        res.push(Diff::Unmodified(super::types::Element::note(src.notes[(i-1) as usize].clone())));
        return res;
    }
    else if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] >= lcs[(i-1) as usize][j as usize]) {
        let mut res = compute_diff_notes(src, dst, lcs, i, j-1);
        let elem = res.pop();
        match elem {
            None => {
                res.push(Diff::Added(super::types::Element::note(dst.notes[(j-1) as usize].clone())));
            },
            Some(e) => {
                match e {
                    Diff::Removed(m) => {
                        res.push(Diff::Modified(super::types::Element::note(dst.notes[(j-1) as usize].clone()), Box::new(Vec::new())));
                    },
                    _ => {
                        res.push(e);
                        res.push(Diff::Added(super::types::Element::note(dst.notes[(j-1) as usize].clone())));
                    }
                }
            }
        }
        return res;
    }
    else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] < lcs[(i-1) as usize][j as usize]) {
        let mut res = compute_diff_notes(src, dst, lcs, i-1, j);
        res.push(Diff::Removed(super::types::Element::note(src.notes[(i-1) as usize].clone())));
        return res;
    }
    else {
        return Vec::new();
    }
}

pub fn compute_diff_part(src: &super::types::Part, dst: &super::types::Part, lcs: Vec<Vec<u32>>, i: isize, j: isize, last_add: bool) -> Vec<Diff> {
    let mut b = i > 0 && j > 0 && src.measures[(i-1) as usize].notes.len() == dst.measures[(j-1) as usize].notes.len();
    if b {
        for ni in 0..src.measures[(i-1) as usize].notes.len() {
            // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
            b = b && (src.measures[(i-1) as usize].notes[ni].pitch == dst.measures[(j-1) as usize].notes[ni].pitch)
            && (src.measures[(i-1) as usize].notes[ni].typee == dst.measures[(j-1) as usize].notes[ni].typee);
        }
    }
    if b {
        let mut res = compute_diff_part(src, dst, lcs, i-1, j-1, false);
        res.push(Diff::Unmodified(super::types::Element::mesure(src.measures[(i-1) as usize].clone())));
        return res;
    }
    if !last_add {
        if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] >= lcs[(i-1) as usize][j as usize]) {
            let mut res = compute_diff_part(src, dst, lcs, i, j-1, true);
            if i == 0 {
                res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
            }
            else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
                    },
                    Some(e) => {
                        match e {
                            Diff::Removed(super::types::Element::mesure(m)) => {
                                let lcs_notes = LCSNotesLength(&m, &(dst.measures[(j-1) as usize]));
                                let i_notes = (lcs_notes.len() as isize) - 1;
                                let j_notes = (lcs_notes[0].len() as isize) - 1;
                                let modified_notes = compute_diff_notes(
                                    &m, 
                                    &(dst.measures[(j-1) as usize]),
                                    lcs_notes, 
                                    i_notes, 
                                    j_notes);
                                res.push(Diff::Modified(super::types::Element::mesure(src.measures[(i-1) as usize].clone()), Box::new(modified_notes)))
                            },
                            _ => {
                                res.push(e);
                                res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
                            }
                        }
                    }
                }
            }
            return res;
        }
        else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] < lcs[(i-1) as usize][j as usize]) {
            let mut res = compute_diff_part(src, dst, lcs, i-1, j, false);
            res.push(Diff::Removed(super::types::Element::mesure(src.measures[(i-1) as usize].clone())));
            return res;
        }
        else {
            return Vec::new();
        }
    }
    else {
        if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] > lcs[(i-1) as usize][j as usize]) {
            let mut res = compute_diff_part(src, dst, lcs, i, j-1, true);
            if i == 0 {
                res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
            }
            else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
                    },
                    Some(e) => {
                        match e {
                            Diff::Removed(super::types::Element::mesure(m)) => {
                                let lcs_notes = LCSNotesLength(&m, &(dst.measures[(j-1) as usize]));
                                let i_notes = (lcs_notes.len() as isize) - 1;
                                let j_notes = (lcs_notes[0].len() as isize) - 1;
                                let modified_notes = compute_diff_notes(
                                    &m, 
                                    &(dst.measures[(j-1) as usize]),
                                    lcs_notes, 
                                    i_notes, 
                                    j_notes);
                                res.push(Diff::Modified(super::types::Element::mesure(src.measures[(i-1) as usize].clone()), Box::new(modified_notes)))
                            },
                            _ => {
                                res.push(e);
                                res.push(Diff::Added(super::types::Element::mesure(dst.measures[(j-1) as usize].clone())));
                            }
                        }
                    }
                }
            }
            return res;
        }
        else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] <= lcs[(i-1) as usize][j as usize]) {
            let mut res = compute_diff_part(src, dst, lcs, i-1, j, false);
            res.push(Diff::Removed(super::types::Element::mesure(src.measures[(i-1) as usize].clone())));
            return res;
        }
        else {
            return Vec::new();
        }
    }
}

pub fn print_diff(src: &super::types::ScorePartwise, dst: &super::types::ScorePartwise, lcs : Vec<Vec<u32>>, i: isize, j: isize, last_add: bool) {
    let mut b = i > 0 && j > 0 && src.parts[0].measures[(i-1) as usize].notes.len() == dst.parts[0].measures[(j-1) as usize].notes.len();
    if b {
        for ni in 0..src.parts[0].measures[(i-1) as usize].notes.len() {
            // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
            b = b && (src.parts[0].measures[(i-1) as usize].notes[ni].pitch == dst.parts[0].measures[(j-1) as usize].notes[ni].pitch)
            && (src.parts[0].measures[(i-1) as usize].notes[ni].typee == dst.parts[0].measures[(j-1) as usize].notes[ni].typee);
        }
    }
    if b {
        print_diff(src, dst, lcs, i-1, j-1, false);
        println!(" {:#?}", src.parts[0].measures[(i-1) as usize]);
    }
    else if !last_add {
        if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] >= lcs[(i-1) as usize][j as usize]) {
            print_diff(src, dst, lcs, i, j-1, true);
            println!("+ {:#?}", dst.parts[0].measures[(j-1) as usize]);
        }
        else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] < lcs[(i-1) as usize][j as usize]) {
            print_diff(src, dst, lcs, i-1, j, false);
            println!("- {:#?}", src.parts[0].measures[(i-1) as usize]);
        }
        else {
            println!("");
        }
    }
    else {
        if j > 0 && (i == 0 || lcs[i as usize][(j-1) as usize] > lcs[(i-1) as usize][j as usize]) {
            print_diff(src, dst, lcs, i, j-1, true);
            println!("+ {:#?}", dst.parts[0].measures[(j-1) as usize]);
        }
        else if i > 0 && (j == 0 || lcs[i as usize][(j-1) as usize] <= lcs[(i-1) as usize][j as usize]) {
            print_diff(src, dst, lcs, i-1, j, false);
            println!("- {:#?}", src.parts[0].measures[(i-1) as usize]);
        }
        else {
            println!("");
        }
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
    // println!("{:#?}", lcs);
    // print_diff(src, dst, lcs, i, j, false);
    println!("{:#?}", compute_diff_part(&(src.parts[0]), &(dst.parts[0]), lcs, i, j, false));
    return res;
    

    // TODO : traiter le cas de quand on fait un retrait, voir si on n'a pas fait d'ajout juste avant ==> modification
    // TODO : le diff sur les notes n'est pas bon, il faut le refaire
}