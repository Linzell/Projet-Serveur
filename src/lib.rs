use std::thread;
use std::sync::mpsc;

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Mission>,
}

struct Mission;

impl GroupeTaches {
    pub fn new(taille: usize) -> GroupeTaches {
        assert!(taille > 0);

        let (envoi, reception) = mpsc::channel();

        let mut operateurs = Vec::with_capacity(taille);

        for id in 0..taille {
            operateurs.push(Operateur::new(id));
        }

        GroupeTaches { operateurs, envoi }
    }

    pub fn executer<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Operateur {
    id: usize,
    tache: thread::JoinHandle<()>,
}

impl Operateur {
    fn new(id: usize) -> Operateur {
        let tache = thread::spawn(|| {});

        Operateur { id, tache }
    }
}
