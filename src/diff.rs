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
    for k in 0..(i - 1) {
        c[k][0] = 0;
    }
    for k in 0..(j - 1) {
        c[0][k] = 0;
    }
    for k in 1..i {
        for l in 1..j {
            if (src.notes[k - 1].typee == dst.notes[l - 1].typee)
                && (src.notes[k - 1].pitch == dst.notes[l - 1].pitch)
                && (src.notes[k - 1].voice == dst.notes[l - 1].voice)
                && (src.notes[k - 1].staff == dst.notes[l - 1].staff)
                && (src.notes[k - 1].dot == dst.notes[l - 1].dot)
            {
                c[k][l] = c[k - 1][l - 1] + 1
            } else {
                c[k][l] = std::cmp::max(c[k][l - 1], c[k - 1][l])
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
    for k in 0..(i - 1) {
        c[k][0] = 0;
    }
    for k in 0..(j - 1) {
        c[0][k] = 0;
    }
    for k in 1..i {
        for l in 1..j {
            longestDivisionSrc = match &src.measures[k - 1].attributes {
                None => longestDivisionSrc,
                Some(a) => a.divisions as f32,
            };
            longestDivisionDst = match &dst.measures[l - 1].attributes {
                None => longestDivisionDst,
                Some(a) => a.divisions as f32,
            };
            let mut b = true;
            if src.measures[k - 1].notes.len() == dst.measures[l - 1].notes.len() {
                for ni in 0..src.measures[k - 1].notes.len() {
                    // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
                    b = b
                        && (src.measures[k - 1].notes[ni].pitch
                            == dst.measures[l - 1].notes[ni].pitch)
                        && (src.measures[k - 1].notes[ni].typee
                            == dst.measures[l - 1].notes[ni].typee)
                        && (src.measures[k - 1].notes[ni].voice
                            == dst.measures[l - 1].notes[ni].voice)
                        && (src.measures[k - 1].notes[ni].staff
                            == dst.measures[l - 1].notes[ni].staff)
                        && (src.measures[k - 1].notes[ni].is_chord
                            == dst.measures[l - 1].notes[ni].is_chord)
                        && (src.measures[k - 1].notes[ni].dot == dst.measures[l - 1].notes[ni].dot)
                        && (src.measures[k - 1].notes[ni].duration as f32 / longestDivisionSrc
                            == dst.measures[l - 1].notes[ni].duration as f32 / longestDivisionDst)
                }
                if b {
                    c[k][l] = c[k - 1][l - 1] + 1
                }
                else {
                    c[k][l] = std::cmp::max(c[k][l - 1], c[k - 1][l]);
                }
            } else {
                c[k][l] = std::cmp::max(c[k][l - 1], c[k - 1][l]);
            }
        }
    }
    return c;
}

pub fn compute_diff_notes(
    src: &super::types::Mesure,
    dst: &super::types::Mesure,
    lcs: Vec<Vec<u32>>,
    i: isize,
    j: isize,
    last_add: bool,
) -> Vec<Diff> {
    if i > 0
        && j > 0
        && (src.notes[(i - 1) as usize].pitch == dst.notes[(j - 1) as usize].pitch)
        && (src.notes[(i - 1) as usize].typee == dst.notes[(j - 1) as usize].typee)
    {
        let mut res = compute_diff_notes(src, dst, lcs, i - 1, j - 1, false);
        res.push(Diff::Unmodified(super::types::Element::note(
            src.notes[(i - 1) as usize].clone(),
        )));
        return res;
    }
    if !last_add {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] >= lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_notes(src, dst, lcs, i, j - 1, true);
            let elem = res.pop();
            match elem {
                None => {
                    res.push(Diff::Added(super::types::Element::note(
                        dst.notes[(j - 1) as usize].clone(),
                    )));
                }
                Some(e) => match e {
                    Diff::Removed(m) => {
                        res.push(Diff::Modified(
                            super::types::Element::note(dst.notes[(j - 1) as usize].clone()),
                            Box::new(Vec::new()),
                        ));
                    }
                    _ => {
                        res.push(e);
                        res.push(Diff::Added(super::types::Element::note(
                            dst.notes[(j - 1) as usize].clone(),
                        )));
                    }
                },
            }
            return res;
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] < lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_notes(src, dst, lcs, i - 1, j, false);
            if j == 0 {
                res.push(Diff::Removed(super::types::Element::note(
                    src.notes[(i - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Removed(super::types::Element::note(
                            src.notes[(i - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Added(m) => {
                            res.push(Diff::Modified(
                                super::types::Element::note(dst.notes[(j - 1) as usize].clone()),
                                Box::new(Vec::new()),
                            ));
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Removed(super::types::Element::note(
                                src.notes[(i - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else {
            return Vec::new();
        }
    } else {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] > lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_notes(src, dst, lcs, i, j - 1, true);
            let elem = res.pop();
            match elem {
                None => {
                    res.push(Diff::Added(super::types::Element::note(
                        dst.notes[(j - 1) as usize].clone(),
                    )));
                }
                Some(e) => match e {
                    Diff::Removed(m) => {
                        res.push(Diff::Modified(
                            super::types::Element::note(dst.notes[(j - 1) as usize].clone()),
                            Box::new(Vec::new()),
                        ));
                    }
                    _ => {
                        res.push(e);
                        res.push(Diff::Added(super::types::Element::note(
                            dst.notes[(j - 1) as usize].clone(),
                        )));
                    }
                },
            }
            return res;
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] <= lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_notes(src, dst, lcs, i - 1, j, false);
            if j == 0 {
                res.push(Diff::Removed(super::types::Element::note(
                    src.notes[(i - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Removed(super::types::Element::note(
                            src.notes[(i - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Added(m) => {
                            res.push(Diff::Modified(
                                super::types::Element::note(dst.notes[(j - 1) as usize].clone()),
                                Box::new(Vec::new()),
                            ));
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Removed(super::types::Element::note(
                                src.notes[(i - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else {
            return Vec::new();
        }
    }
}

pub fn compute_diff_part(
    src: &super::types::Part,
    dst: &super::types::Part,
    lcs: Vec<Vec<u32>>,
    i: isize,
    j: isize,
    last_add: bool,
) -> Vec<Diff> {
    let mut b = i > 0
        && j > 0
        && src.measures[(i - 1) as usize].notes.len() == dst.measures[(j - 1) as usize].notes.len();
    if b {
        for ni in 0..src.measures[(i - 1) as usize].notes.len() {
            // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
            b = b
                && (src.measures[(i - 1) as usize].notes[ni].pitch
                    == dst.measures[(j - 1) as usize].notes[ni].pitch)
                && (src.measures[(i - 1) as usize].notes[ni].typee
                    == dst.measures[(j - 1) as usize].notes[ni].typee)
                && (src.measures[(i - 1) as usize].notes[ni].voice
                    == dst.measures[(j - 1) as usize].notes[ni].voice)
                && (src.measures[(i - 1) as usize].notes[ni].dot
                    == dst.measures[(j - 1) as usize].notes[ni].dot)
                && (src.measures[(i - 1) as usize].notes[ni].is_chord
                    == dst.measures[(j - 1) as usize].notes[ni].is_chord)
                && (src.measures[(i - 1) as usize].notes[ni].staff
                    == dst.measures[(j - 1) as usize].notes[ni].staff);
        }
    }
    if b {
        let mut res = compute_diff_part(src, dst, lcs, i - 1, j - 1, false);
        res.push(Diff::Unmodified(super::types::Element::mesure(
            src.measures[(i - 1) as usize].clone(),
        )));
        return res;
    }
    if !last_add {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] >= lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_part(src, dst, lcs, i, j - 1, true);
            if i == 0 {
                res.push(Diff::Added(super::types::Element::mesure(
                    dst.measures[(j - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Added(super::types::Element::mesure(
                            dst.measures[(j - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Removed(super::types::Element::mesure(m)) => {
                            let lcs_notes = LCSNotesLength(&m, &(dst.measures[(j - 1) as usize]));
                            let i_notes = (lcs_notes.len() as isize) - 1;
                            let j_notes = (lcs_notes[0].len() as isize) - 1;
                            let modified_notes = compute_diff_notes(
                                &m,
                                &(dst.measures[(j - 1) as usize]),
                                lcs_notes,
                                i_notes,
                                j_notes,
                                false,
                            );
                            res.push(Diff::Modified(
                                super::types::Element::mesure(
                                    src.measures[(i - 1) as usize].clone(),
                                ),
                                Box::new(modified_notes),
                            ))
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Added(super::types::Element::mesure(
                                dst.measures[(j - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] < lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_part(src, dst, lcs, i - 1, j, false);
            if j == 0 {
                res.push(Diff::Removed(super::types::Element::mesure(
                    src.measures[(i - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Removed(super::types::Element::mesure(
                            src.measures[(i - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Added(super::types::Element::mesure(m)) => {
                            let lcs_notes = LCSNotesLength(&(src.measures[(i - 1) as usize]), &m);
                            let i_notes = (lcs_notes.len() as isize) - 1;
                            let j_notes = (lcs_notes[0].len() as isize) - 1;
                            let modified_notes = compute_diff_notes(
                                &(src.measures[(i - 1) as usize]),
                                &m,
                                lcs_notes,
                                i_notes,
                                j_notes,
                                false,
                            );
                            res.push(Diff::Modified(
                                super::types::Element::mesure(
                                    src.measures[(i - 1) as usize].clone(),
                                ),
                                Box::new(modified_notes),
                            ))
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Removed(super::types::Element::mesure(
                                src.measures[(i - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else {
            return Vec::new();
        }
    } else {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] > lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_part(src, dst, lcs, i, j - 1, true);
            if i == 0 {
                res.push(Diff::Added(super::types::Element::mesure(
                    dst.measures[(j - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Added(super::types::Element::mesure(
                            dst.measures[(j - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Removed(super::types::Element::mesure(m)) => {
                            let lcs_notes = LCSNotesLength(&m, &(dst.measures[(j - 1) as usize]));
                            let i_notes = (lcs_notes.len() as isize) - 1;
                            let j_notes = (lcs_notes[0].len() as isize) - 1;
                            let modified_notes = compute_diff_notes(
                                &m,
                                &(dst.measures[(j - 1) as usize]),
                                lcs_notes,
                                i_notes,
                                j_notes,
                                false,
                            );
                            res.push(Diff::Modified(
                                super::types::Element::mesure(
                                    src.measures[(i - 1) as usize].clone(),
                                ),
                                Box::new(modified_notes),
                            ))
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Added(super::types::Element::mesure(
                                dst.measures[(j - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] <= lcs[(i - 1) as usize][j as usize])
        {
            let mut res = compute_diff_part(src, dst, lcs, i - 1, j, false);
            if j == 0 {
                res.push(Diff::Removed(super::types::Element::mesure(
                    src.measures[(i - 1) as usize].clone(),
                )));
            } else {
                let elem = res.pop();
                match elem {
                    None => {
                        res.push(Diff::Removed(super::types::Element::mesure(
                            src.measures[(i - 1) as usize].clone(),
                        )));
                    }
                    Some(e) => match e {
                        Diff::Added(super::types::Element::mesure(m)) => {
                            let lcs_notes = LCSNotesLength(&(src.measures[(i - 1) as usize]), &m);
                            let i_notes = (lcs_notes.len() as isize) - 1;
                            let j_notes = (lcs_notes[0].len() as isize) - 1;
                            let modified_notes = compute_diff_notes(
                                &(src.measures[(i - 1) as usize]),
                                &m,
                                lcs_notes,
                                i_notes,
                                j_notes,
                                false,
                            );
                            res.push(Diff::Modified(
                                super::types::Element::mesure(
                                    src.measures[(i - 1) as usize].clone(),
                                ),
                                Box::new(modified_notes),
                            ))
                        }
                        _ => {
                            res.push(e);
                            res.push(Diff::Removed(super::types::Element::mesure(
                                src.measures[(i - 1) as usize].clone(),
                            )));
                        }
                    },
                }
            }
            return res;
        } else {
            return Vec::new();
        }
    }
}

pub fn print_diff(
    src: &super::types::ScorePartwise,
    dst: &super::types::ScorePartwise,
    lcs: Vec<Vec<u32>>,
    i: isize,
    j: isize,
    last_add: bool,
) {
    let mut b = i > 0
        && j > 0
        && src.parts[0].measures[(i - 1) as usize].notes.len()
            == dst.parts[0].measures[(j - 1) as usize].notes.len();
    if b {
        for ni in 0..src.parts[0].measures[(i - 1) as usize].notes.len() {
            // Remplacer ce gros truc moche par une fonction d'égalité sur les notes
            b = b
                && (src.parts[0].measures[(i - 1) as usize].notes[ni].pitch
                    == dst.parts[0].measures[(j - 1) as usize].notes[ni].pitch)
                && (src.parts[0].measures[(i - 1) as usize].notes[ni].typee
                    == dst.parts[0].measures[(j - 1) as usize].notes[ni].typee);
        }
    }
    if b {
        print_diff(src, dst, lcs, i - 1, j - 1, false);
        println!(" {:#?}", src.parts[0].measures[(i - 1) as usize]);
    } else if !last_add {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] >= lcs[(i - 1) as usize][j as usize])
        {
            print_diff(src, dst, lcs, i, j - 1, true);
            println!("+ {:#?}", dst.parts[0].measures[(j - 1) as usize]);
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] < lcs[(i - 1) as usize][j as usize])
        {
            print_diff(src, dst, lcs, i - 1, j, false);
            println!("- {:#?}", src.parts[0].measures[(i - 1) as usize]);
        } else {
            println!("");
        }
    } else {
        if j > 0
            && (i == 0 || lcs[i as usize][(j - 1) as usize] > lcs[(i - 1) as usize][j as usize])
        {
            print_diff(src, dst, lcs, i, j - 1, true);
            println!("+ {:#?}", dst.parts[0].measures[(j - 1) as usize]);
        } else if i > 0
            && (j == 0 || lcs[i as usize][(j - 1) as usize] <= lcs[(i - 1) as usize][j as usize])
        {
            print_diff(src, dst, lcs, i - 1, j, false);
            println!("- {:#?}", src.parts[0].measures[(i - 1) as usize]);
        } else {
            println!("");
        }
    }
}

pub fn diff(
    src: &super::types::ScorePartwise,
    dst: &super::types::ScorePartwise,
) -> Vec<Vec<Diff>> {
    let mut res = Vec::new();
    for k in 0..std::cmp::max(src.parts.len(), dst.parts.len()) {
        if k >= src.parts.len() {
            let mut p = Vec::new();
            dst.parts[k].measures.iter().for_each(|x| p.push(Diff::Added(super::types::Element::mesure(x.clone()))));
            res.push(p);
        } else {
            let lcs = LCSMeasuresLength(&src.parts[k], &dst.parts[k]);
            let i: isize = (lcs.len() as isize) - 1;
            let j: isize = (lcs[0].len() as isize) - 1;
            res.push(compute_diff_part(
                &(src.parts[k]),
                &(dst.parts[k]),
                lcs,
                i,
                j,
                false,
            ));
        }
    }
    return res;
}
