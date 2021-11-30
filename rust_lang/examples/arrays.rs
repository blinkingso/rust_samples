use std::any::Any;

/// Rust Arrays;
fn main() {}

#[test]
fn test_create() {
    // initialize an array;
    let arr = [1, 2, 3, 4u32];
    // create an empty array;
    let mut arr = [0isize; 32];
    arr[0] = 1000;

    let blank2: [u8; 3] = [1, 2, 3];
    // compile error here: ^^^^^^ expected `isize`, found `u8`
    // let arrays = [arr, blank2];
    // println!("{:?}", arrays);
    let blank = [0u8; 3];
    let arrays = [blank, blank2];
    println!("{:?}", arrays);
}

///slice
#[test]
fn test_slice_of_array() {
    let arr = [1, 2, 3, 4];
    let s = &arr[..];
    println!("{}", s.len());
    let sum = IntoIterator::into_iter(arr)
        .map(|x| x * x)
        .fold(0, |x, y| x + y);
    println!("sum^2 is : {}", sum);
    {
        let m = s[0];
        println!("m is {}, {}", m, arr[1]);
    }
}

#[test]
fn test_vectors() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
