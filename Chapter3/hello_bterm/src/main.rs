use bracket_lib::prelude::*;

struct State { }

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm ) {
        ctx.cls();
        ctx.print(1,1,"Hello,游戏开发者们"); //中文不能显示
    }
}


fn main()  -> BError {

    let context = BTermBuilder::simple80x50()
        .with_title("坠落的小鸟")
        .build()?;

    main_loop(context,State{})
}
