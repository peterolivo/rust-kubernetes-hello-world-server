#![allow(unreachable_code)]
use rouille::router;

fn fib() -> u128 {
    let mut a: u64 = 0;
    let mut b = 1;
    for l in 0..10_000_000 {
        println!("{}", l);
        let c = std::hint::black_box(a + b);
        a = std::hint::black_box(b);
        b = std::hint::black_box(c);
    }
    let d = 1;
    return d;
}

fn get_frog() -> &'static str {
    r#"
                               .-----.
                              /7  .  (
                             /   .-.  \
                            /   /   \  \
                           / `  )   (   )
                          / `   )   ).  \
                        .'  _.   \_/  . |
       .--.           .' _.' )`.        |
      (    `---...._.'   `---.'_)    ..  \
       \            `----....___    `. \  |
        `.           _ ----- _   `._  )/  |
          `.       /"  \   /"  \`.  `._   |
            `.    ((O)` ) ((O)` ) `.   `._\
              `-- '`---'   `---' )  `.    `-.
                 /                  ` \      `-.
               .'                      `.       `.
              /                     `  ` `.       `-.
       .--.   \ ===._____.======. `    `   `. .___.--`     .''''.
      ' .` `-. `.                )`. `   ` ` \          .' . '  8)
     (8  .  ` `-.`.               ( .  ` `  .`\      .'  '    ' /
      \  `. `    `-.               ) ` .   ` ` \  .'   ' .  '  /
       \ ` `.  ` . \`.    .--.     |  ` ) `   .``/   '  // .  /
        `.  ``. .   \ \   .-- `.  (  ` /_   ` . / ' .  '/   .'
          `. ` \  `  \ \  '-.   `-'  .'  `-.  `   .  .'/  .'
            \ `.`.  ` \ \    ) /`._.`       `.  ` .  .'  /
      PRO    |  `.`. . \ \  (.'               `.   .'  .'
          __/  .. \ \ ` ) \                     \.' .. \__
   .-._.-'     '"  ) .-'   `.                   (  '"     `-._.--.
  (_________.-====' / .' /\_)`--..__________..-- `====-. _________)
                   (.'(.'`
"#
}

fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/hello/world")
            },

            (GET) (/frog) => {
                rouille::Response::text(get_frog())
            },

            (GET) (/hello/world) => {
                println!("hello world");

                rouille::Response::text("hello world")
            },

            (GET) (/panic) => {
                panic!("Oops!")
            },

            (GET) (/fib) => {
                fib();

                rouille::Response::text(format!("complete fib calc"))
            },

            (GET) (/{id: u32}) => {
                println!("u32 {:?}", id);

                rouille::Response::empty_400()
            },

            (GET) (/{id: String}) => {
                println!("String {:?}", id);

                rouille::Response::text(format!("hello, {}", id))
            },

            _ => rouille::Response::empty_404()
            )
        }
    )
}
