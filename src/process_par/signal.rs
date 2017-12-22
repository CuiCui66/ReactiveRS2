use node::*;
use super::*;

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/

impl<'a, In: 'a, E: 'a, S: 'a> ProcessPar<'a, ((S, E),In)> for EmitD
where
    S: Signal<'a, E=E> + Send + Sync,
    In: Sync + Send,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, E: 'a, S: 'a> ProcessPar<'a, (S, E)> for EmitD
where
    S: Signal<'a, E = E> + Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, In: 'a, E: 'a, S: 'a> ProcessPar<'a, (Vec<(S, E)>,In)> for EmitD
where
    S: Signal<'a, E=E> + Send + Sync,
    In: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, E: 'a, S: 'a> ProcessPar<'a, Vec<(S, E)>> for EmitD
where
    S: Signal<'a, E = E> + Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitD {}
    }
}

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_/ ___|
// |  _| | '_ ` _ \| | __\___ \
// | |___| | | | | | | |_ ___) |
// |_____|_| |_| |_|_|\__|____/


impl<'a, E: 'a, S: 'a> ProcessPar<'a, E> for EmitS<S, E>
where
    S: Signal<'a, E = E> + Send + Sync,
    E: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitS<S, E>;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
}

impl<'a, S: 'a, E: 'a, In: 'a> ProcessPar<'a, (E,In)> for EmitS<S, E>
where
    S: Signal<'a, E = E> + Send + Sync,
    E: Send + Sync,
    In: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type NIO = NEmitS<S, E>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
}


//  _____           _ _ __     __        ____
// | ____|_ __ ___ (_) |\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/ \___|\___|____/


impl<'a, E: 'a, S: 'a> ProcessPar<'a, Vec<E>> for EmitVecS<S>
where
    S: Signal<'a, E = E> + Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitVecS<S>;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }

}

impl<'a, S: 'a, E: 'a, In: 'a> ProcessPar<'a, (Vec<E>,In)> for EmitVecS<S>
where
    S: Signal<'a, E = E> + Send + Sync,
    In: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type NIO = NEmitVecS<S>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


//  _____           _ _ __     ______
// | ____|_ __ ___ (_) |\ \   / / ___|
// |  _| | '_ ` _ \| | __\ \ / /\___ \
// | |___| | | | | | | |_ \ V /  ___) |
// |_____|_| |_| |_|_|\__| \_/  |____/



impl<'a, In: 'a, E: 'a, S: 'a> ProcessPar<'a, In> for EmitVS<S, E>
where
    S: Signal<'a, E = E> + Send + Sync,
    E: Clone + Send + Sync,
    In: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVS<S, E>;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitVS(self.0, self.1)
    }

}


//  _____           _ _ __     ____     __        ____
// | ____|_ __ ___ (_) |\ \   / /\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / /  \ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /    \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/      \_/ \___|\___|____/


impl<'a, In: 'a, E: 'a, S: 'a> ProcessPar<'a, In> for EmitVVecS<S, E>
where
    S: Signal<'a, E = E> + Send + Sync,
    E: Clone + Send + Sync,
    In: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVVecS<S, E>;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }

}


//     _                _ _   ____
//    / \__      ____ _(_) |_|  _ \
//   / _ \ \ /\ / / _` | | __| | | |
//  / ___ \ V  V / (_| | | |_| |_| |
// /_/   \_\_/\_/ \__,_|_|\__|____/


impl<'a, V: 'a, S: 'a> ProcessPar<'a, S> for AwaitD
where
    S: Signal<'a, V = V> + Send + Sync,
    V: Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<V>;
    type NI = NSeq<NAwaitD, ArcStore<S>>;
    type NO = NSeq<ArcLoad<S>, NGetD>;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_amutex();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store_par(rc));
        let no = node!(load_par(rc2) >> NGetD {});
        (ni, out_id, no)
    }
}


impl<'a, In: 'a, V: 'a, S: 'a> ProcessPar<'a, (S, In)> for AwaitD
where
    S: Signal<'a, V=V> + Send + Sync,
    V: Send + Sync,
    In: Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<NPar<NAwaitD,NIdentity>,ArcStore<(S, In)>>;
    type NO = NSeq<ArcLoad<(S, In)>, NGetD>;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_amutex();
        let rc2 = rc.clone();

        // Type inference won't work here
        let ni_first = <NAwaitD as Node<'a,S>>::njoin::<In, NIdentity>(NAwaitD(out_id), NIdentity {});
        let ni = node!(ni_first >> store_par(rc));
        let no = node!(load_par(rc2) >> NGetD{});
        (ni, out_id, no)
    }
}

//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/


impl<'a, In: 'a, V: 'a, S: 'a> ProcessPar<'a, In> for AwaitS<S>
where
    S: Signal<'a, V=V> + Send + Sync,
    V: Send + Sync,
    In: Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<ArcStore<In>,NAwaitS<S>>;
    type NO = NSeq<GenP, NPar<NGetS<S>, ArcLoad<In>>>;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_amutex();
        let rc2 = rc.clone();

        let ni = node!(store_par(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP >> (NGetS(self.0) || load_par(rc2)));
        (ni, out_id, no)
    }
}


//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


impl<'a, S: 'a> ProcessPar<'a, S> for AwaitImmediateD
where
    S: Signal<'a> + Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<()>;
    type NI = NAwaitImmediateD;
    type NO = Nothing;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }
}

impl<'a, In: 'a, S: 'a> ProcessPar<'a, (S, In)> for AwaitImmediateD
where
    S: Signal<'a> + Send + Sync,
    In: Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<NSeq<NPar<NIdentity,ArcStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = ArcLoad<In>;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_amutex();
        let rc2 = rc.clone();

        let ni_first = <NIdentity as Node<'a,S>>::njoin::<In, ArcStore<In>>(NIdentity {}, store_par(rc));
        let ni_second = <NPar<NIdentity,ArcStore<In>> as Node<'a, (S, In)>>::nseq(ni_first, Ignore2);
        let ni = <NSeq<NPar<NIdentity,ArcStore<In>>,Ignore2> as Node<'a, (S, In)>>::nseq(ni_second, NAwaitImmediateD(out_id));
        let no = load_par(rc2);
        (ni, out_id, no)
    }
}

//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


impl<'a, In: 'a, V: 'a, S: 'a> ProcessPar<'a, In> for AwaitImmediateS<S>
where
    S: Signal<'a, V=V> + Send + Sync,
    In: Send + Sync,
{
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<ArcStore<In>,NAwaitImmediateS<S>>;
    type NO = ArcLoad<In>;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_amutex();
        let rc2 = rc.clone();

        let ni = node!(store_par(rc) >> NAwaitImmediateS(self.0.clone(), out_id));
        let no = load_par(rc2);
        (ni, out_id, no)
    }
}

//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_|  _ \
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __| | | |
// |  __/| | |  __/\__ \  __/ | | | |_| |_| |
// |_|   |_|  \___||___/\___|_| |_|\__|____/


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, S>
    for PresentD<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, NotIm>>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentD;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let (ptni, ptind, ptno) = self.pt.p.compile_par(g);
        let (pfni, pfind, pfno) = self.pf.p.compile_par(g);

        let out_id = g.reserve();
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out_id)));
        let nit_id = g.add(box ptni);
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: nif_id,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, S>
for PresentD<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, NotIm>>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentD;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let ptnio = self.pt.p.compileIm_par(g);
        let (pfni, pfind, pfno) = self.pf.p.compile_par(g);

        let out_id = g.reserve();
        let ptind = g.add(box node!(ptnio >> store_par(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out_id)));
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: ptind,
            node_false: nif_id,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, S>
for PresentD<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, IsIm>>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentD;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm_par(g);
        let (ptni, ptind, ptno) = self.pt.p.compile_par(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store_par(rcf) >> jump(out_id)));
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out_id)));
        let nit_id = g.add(box ptni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: pfind,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, S>
for PresentD<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, IsIm>>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentD;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm_par(g);
        let ptnio = self.pt.p.compileIm_par(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store_par(rcf) >> jump(out_id)));
        let ptind = g.add(box node!(ptnio >> store_par(rct) >> jump(out_id)));

        let ni = NPresentD {
            node_true: ptind,
            node_false: pfind,
        };

        (ni, out_id, load_par(rc_out))
    }
}

//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_/ ___|
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __\___ \
// |  __/| | |  __/\__ \  __/ | | | |_ ___) |
// |_|   |_|  \___||___/\___|_| |_|\__|____/


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, ()>
for PresentS<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, NotIm>, S>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentS<S>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let (ptni, ptind, ptno) = self.pt.p.compile_par(g);
        let (pfni, pfind, pfno) = self.pf.p.compile_par(g);

        let out_id = g.reserve();
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out_id)));
        let nit_id = g.add(box ptni);
        let nif_id = g.add(box pfni);

        let ni = NPresentS {
            node_true: nit_id,
            node_false: nif_id,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, ()>
for PresentS<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, NotIm>, S>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentS<S>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let ptnio = self.pt.p.compileIm_par(g);
        let (pfni, pfind, pfno) = self.pf.p.compile_par(g);

        let out_id = g.reserve();
        let ptind = g.add(box node!(ptnio >> store_par(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out_id)));
        let nif_id = g.add(box pfni);

        let ni = NPresentS {
            node_true: ptind,
            node_false: nif_id,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, ()>
for PresentS<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, IsIm>, S>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentS<S>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm_par(g);
        let (ptni, ptind, ptno) = self.pt.p.compile_par(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store_par(rcf) >> jump(out_id)));
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out_id)));
        let nit_id = g.add(box ptni);

        let ni = NPresentS {
            node_true: nit_id,
            node_false: pfind,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load_par(rc_out))
    }
}


impl<'a, PT, PF, S: 'a, Out: 'a> ProcessPar<'a, ()>
for PresentS<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, IsIm>, S>
where
    PT: ProcessPar<'a, (), Out=Out>,
    PF: ProcessPar<'a, (), Out=Out>,
    S: Signal<'a> + Send + Sync,
    Out: Send + Sync,
{
    type NI = NPresentS<S>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_amutex();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm_par(g);
        let ptnio = self.pt.p.compileIm_par(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store_par(rcf) >> jump(out_id)));
        let ptind = g.add(box node!(ptnio >> store_par(rct) >> jump(out_id)));

        let ni = NPresentS {
            node_true: ptind,
            node_false: pfind,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load_par(rc_out))
    }
}
