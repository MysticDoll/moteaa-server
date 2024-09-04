type Motemen = [[(u8, u8, u8); 4]; 4];
pub(crate) const MOTEMEN: [[(u8, u8, u8); 4]; 4] = [
    [( 195, 213, 227 ), ( 252, 255, 246 ), ( 19, 58, 137 ),   ( 157, 167, 185 )],
    [( 129, 82, 38 ),   ( 255, 224, 211 ), ( 248, 199, 182 ), ( 218, 46, 32 )],
    [( 164, 172, 193 ), ( 254, 196, 184 ), ( 255, 179, 162 ), ( 225, 107, 93 )],
    [( 174, 179, 183 ), ( 55, 62, 68 ), ( 83, 32, 13 ),    ( 89, 90, 95 )]
];

pub(crate) struct Svg {
    pub content: String
}
pub(crate) struct Html {
    pub content: String
}

pub(crate) struct Shell {
    pub content: String
}

pub(crate) enum MotemenVariant {
    HTML(Html),
    SHELL(Shell),
    SVG(Svg)
}

impl Into<Shell> for [[(u8, u8, u8); 4]; 4] {
    fn into(self) -> Shell {
        (MOTEMEN, 1).into()
    }
}

impl Into<Html> for [[(u8,u8,u8); 4]; 4] {
    fn into(self) -> Html {
        (MOTEMEN, 1).into()
    }
}

impl Into<Svg> for [[(u8,u8,u8); 4]; 4] {
    fn into(self) -> Svg {
        (MOTEMEN, 1).into()
    }
}

impl Into<Svg> for ( [[(u8, u8, u8); 4]; 4], u8 ) {
    fn into(self) -> Svg {
        let size = self.1;
        let content = self.0
            .into_iter()
            .enumerate()
            .map(|(y, row)| 
                 row
                 .into_iter()
                 .enumerate()
                 .map(|(x, color)|
                      format!(
                          "<rect x=\"{}em\" y=\"{}em\" width=\"{}em\" height=\"{}em\" stroke=\"rgb({}, {}, {})\" fill=\"rgb({}, {}, {})\" />",
                          x as u32 * size as u32,
                          y as u32 * size as u32,
                          size,
                          size,
                          color.0,
                          color.1,
                          color.2,
                          color.0,
                          color.1,
                          color.2
                      )
                 )
                 .collect::<Vec<String>>()
                 .concat()
            )
            .collect::<Vec<String>>()
            .concat();
        Svg {
            content: format!("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}em\" height=\"{}em\">{}</svg>", size as u32 * 4, size as u32 * 4, content)
        }
    }
}

impl Into<Html> for ( [[(u8, u8, u8); 4]; 4], u8 ) {
    fn into(self) -> Html {
        let size = self.1;
        let content = self.0
            .into_iter()
            .map(|row| 
                 row
                 .into_iter()
                 .map(|color|
                      format!(
                          "<span class=\"motemen-cell\" style=\"background-color: rgb({}, {}, {});\">  </span>",
                          color.0,
                          color.1,
                          color.2
                      )
                 )
                 .collect::<Vec<String>>()
                 .concat()
            )
            .map(|row| 
                 format!(
                     "<p style=\"line-height: 0px; margin: 0px;\">{}</p>",
                     row
                 )
            )
            .collect::<Vec<String>>()
            .concat();
        Html {
            content: format!("<style>.motemen-cell {{height: {}em; width: {}em; display: inline-block; }}</style> {}", size, size, content)
        }
    }
}

impl Into<Shell> for ( [[(u8, u8, u8); 4]; 4], u8 ) {
    fn into(self) -> Shell {
        let size: usize = self.1.into();
        let content = self.0
            .into_iter()
            .map(|row| {
                vec![
                    format!("{}\x1b[m\n", row
                            .into_iter()
                            .map(|color| format!(
                                "{}{}",
                                format!("\x1b[48;2;{};{};{}m", color.0, color.1, color.2),
                                vec!["  ".to_owned(); size].concat()
                            ))
                            .collect::<Vec<String>>()
                            .concat());
                    size 
                ].concat()
            })
            .collect::<Vec<String>>()
            .concat();
        Shell {
            content 
        }
    }
}

impl Into<MotemenVariant> for String {
    fn into(self) -> MotemenVariant {
        (self, 1).into()
    }
}

impl Into<MotemenVariant> for (String, Motemen) {
    fn into(self) -> MotemenVariant {
        let (format, motemen) = self;
        (format, motemen, 1).into()
    }
}

impl Into<MotemenVariant> for (String, u8) {
    fn into(self) -> MotemenVariant {
        let (format, size) = self;
        (format, MOTEMEN, size).into()
    }
}

impl Into<MotemenVariant> for (String, Motemen, u8) {
    fn into(self) -> MotemenVariant {
        let (format, motemen, size) = self;
        match format.as_str() {
            "shell" => MotemenVariant::SHELL((motemen, size).into()),
            "html" => MotemenVariant::HTML((motemen, size).into()),
            "svg" => MotemenVariant::SVG((motemen, size).into()),
            _ => MotemenVariant::SHELL((motemen, size).into()),
        }
    }
}
