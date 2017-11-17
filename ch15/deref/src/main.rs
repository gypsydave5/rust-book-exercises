use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn simple_deref() {
    let mut x = 5;
    {
        let mut y = &mut x;
        *y += 1;
    }

    assert_eq!(x, 6);
}

fn override_deref() {
    let my_mp3 = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("Avril Lavigne")),
        title: Some(String::from("Girlfriend")),
    };

    assert_eq!(vec![1, 2, 3], *my_mp3);
}


fn main() {
    simple_deref();
    override_deref();
}
