trait T1 {}
trait T2 {}
trait T3 {}
trait T4 {}

impl T2 for i32 {}
impl T3 for i32 {}

struct Wrapper<W> {
    value: W,
}
impl<B: T2> T1 for Wrapper<B> {}

struct Burrito<F> {
    spicy: bool,
    filling: F,
}
impl<A: T3> T2 for Burrito<A> {}

struct BurritoTuple<F>(F);
impl<C: T3> T2 for BurritoTuple<C> {}

enum BurritoKinds<G> {
    SmallBurrito { spicy: bool, small_filling: G },
    LargeBurrito { spicy: bool, large_filling: G },
    MultiBurrito { first_filling: G, second_filling: G },
}
impl<D: T3> T2 for BurritoKinds<D> {}

struct Taco<H>(bool, H);
impl<E: T3> T2 for Taco<E> {}

enum TacoKinds<H> {
    OneTaco(bool, H),
    TwoTacos(bool, H, H),
}
impl<F: T3> T2 for TacoKinds<F> {}

fn want<V: T1>(_x: V) {}

fn example<Q>(q: Q) {
    want(Wrapper { value: Burrito { spicy: false, filling: q } });
    //~^ ERROR the trait bound `Q: T3` is not satisfied [E0277]

    want(Wrapper { value: BurritoKinds::SmallBurrito { spicy: true, small_filling: q } });
    //~^ ERROR the trait bound `Q: T3` is not satisfied [E0277]

    want(Wrapper { value: Taco(false, q) });
    //~^ ERROR the trait bound `Q: T3` is not satisfied [E0277]

    want(Wrapper { value: TacoKinds::OneTaco(false, q) });
    //~^ ERROR the trait bound `Q: T3` is not satisfied [E0277]
}

fn main() {}
