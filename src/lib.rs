#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    fn check(text: &str) {
        const LEN: usize = 8;
        assert_eq!(textwrap_11::wrap(text, LEN), textwrap::wrap(text, LEN));
    }

    #[test]
    fn values() {
        check("some short words that will fit on one line");
        check("asuperlongwordwithnoparticularbreakpoints");
        check("a-super-long-word-with-a-bunch-of-dashes-in-it");
        check(&String::from_utf8(vec![41; 1000]).unwrap());
        check(
            r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam tristique malesuada quam, non venenatis nibh sollicitudin nec. Suspendisse finibus felis id sapien lacinia imperdiet. Ut et purus velit. Nunc scelerisque arcu eros, sed vestibulum nisl posuere in. Etiam lacus magna, molestie et lacinia ut, ultrices ullamcorper libero. Quisque aliquam, mi eget bibendum vulputate, elit dui venenatis tortor, sit amet rutrum quam lectus ut mi. Duis non ornare leo. Aliquam ligula sapien, sollicitudin a condimentum sit amet, egestas eu sem.

Quisque venenatis odio sed augue pharetra malesuada. Morbi arcu erat, posuere sit amet nibh ut, laoreet feugiat libero. Donec nec arcu sollicitudin, finibus augue vitae, gravida velit. Sed urna augue, pulvinar quis ante sed, tristique laoreet ex. Nulla elementum, nunc in elementum congue, est nunc viverra mauris, vel posuere eros augue nec odio. Sed quis ullamcorper mauris. Aliquam ante sem, dictum a posuere et, vulputate ut ipsum. Vivamus cursus cursus lacinia. Nulla eget orci scelerisque, varius nulla eu, ultrices velit. Proin felis turpis, mollis in bibendum sed, consectetur quis metus. Fusce convallis tempor vehicula. Morbi pharetra libero efficitur odio lobortis, non aliquam turpis elementum. Proin varius pulvinar sapien. Fusce finibus nulla non odio euismod, dignissim vulputate mauris gravida.

Nunc porta ligula in nunc ultrices congue. Ut tempor laoreet ligula a porta. Fusce pretium, lacus vitae consequat commodo, sapien ante interdum arcu, et scelerisque enim libero in tortor. Cras posuere mauris at nunc pellentesque, ac imperdiet mi ornare. Nullam in tellus ultrices, mattis dolor et, dictum felis. Vivamus metus arcu, laoreet eget tempor in, aliquam sed nisi. In consectetur ligula nisi, ut consectetur velit blandit id. Etiam nec nisi odio. Vivamus aliquet ultrices libero, ut faucibus felis egestas eu. Cras ut nulla iaculis, viverra augue non, efficitur nisl. Nam at mauris auctor, semper nisi nec, efficitur ante. Mauris ac arcu a felis dignissim ultricies non et leo. Pellentesque convallis, quam nec vehicula eleifend, orci sem commodo turpis, id blandit leo sapien ac lectus. Sed porta diam non nisi blandit aliquet. Vestibulum scelerisque tortor urna, at pharetra eros consectetur id.

Quisque vehicula, justo ornare placerat rutrum, nisi justo commodo mi, vel porttitor nisl augue eu diam. Aliquam justo felis, fringilla a pellentesque vel, commodo ac nulla. In dignissim nulla arcu, sed fermentum dolor mollis non. Suspendisse venenatis, ipsum ac pellentesque auctor, lacus lacus auctor dolor, ut posuere mauris ex ut tortor. Sed porta consectetur nunc pellentesque condimentum. Nam eget scelerisque magna. Morbi lacus nibh, finibus eget sollicitudin nec, blandit at odio. Donec congue et enim in porta. Fusce malesuada sapien at nulla vulputate, non pellentesque massa commodo. Morbi varius, lacus at blandit aliquet, neque urna vulputate nibh, a facilisis diam ex non urna. Aliquam nec turpis ut enim placerat facilisis. Nunc elementum euismod augue, quis viverra justo pellentesque vel. Duis euismod dictum enim at volutpat. Ut vitae convallis ligula, et efficitur nibh. Phasellus cursus nisl quis sodales hendrerit. Ut viverra eros ipsum, at consectetur orci laoreet sed.

Morbi imperdiet, eros quis vestibulum tempus, arcu ex pellentesque enim, eget pellentesque tortor mauris sed nunc. Maecenas vulputate tellus nec rhoncus maximus. Curabitur quis finibus erat, eu lobortis lacus. Donec lacinia eu nulla feugiat porttitor. Nam urna mauris, hendrerit vitae risus consectetur, commodo pulvinar arcu. Aliquam ultrices sed massa sed gravida. Sed dignissim augue justo.

"#,
        );
    }
}
