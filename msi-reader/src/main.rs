use std::fmt::{ Debug, Display };

struct MsiDirectoryName<'a> {
    combined: &'a str
}

struct MsiName<'a> {
    combined: &'a str
}

impl<'a> From<&'a str> for MsiDirectoryName<'a> {
    fn from(combined: &'a str) -> Self {        
        MsiDirectoryName
        {
            combined
        }
    }
}

impl<'a> MsiDirectoryName<'a> {
    
    fn source(&self) -> Option<MsiName>
    {
        if let Some(index) = self.combined.find(':')
        {
            Some(MsiName::from(&self.combined[0..index]))
        }
        else
        {
            None
        }        
    }
    
    fn target(&self) -> MsiName
    {
        if let Some(index) = self.combined.find(':')
        {
            MsiName::from(&self.combined[index + 1..])
        }
        else
        {
            MsiName::from(self.combined)
        } 
    }

    fn combined(&self) -> &str {
        self.combined
    }
}

impl<'a> Debug for MsiDirectoryName<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.combined())
    }
}

impl<'a> Display for MsiDirectoryName<'a>
{
    fn fmt (&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if let Some(source) = &self.source()
        {
            write! (fmt, "source = [{}], target = [{}]", source, &self.target())
        }
        else
        {
            write! (fmt, "{}", &self.target())
        }
    }
}

impl<'a> Debug for MsiName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.combined())
    }
}

impl<'a> Display for MsiName<'a> {
    fn fmt (&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if let Some(short) = &self.short()
        {
            write! (fmt, "short = {}, long = {}", short, self.long())
        }
        else
        {
            write! (fmt, "{}", &self.long())
        }
    }
}

impl<'a> From<&'a str> for MsiName<'a> {
    fn from(combined: &'a str) -> Self    
    {
        MsiName {
            combined
        }
    }
}

impl<'a> MsiName<'a> {

    fn long(&self) -> &str {
        if let Some(index) = self.combined.find('|')
        {
            &self.combined[index + 1..]
        }
        else
        {
            &self.combined
        }
    }

    fn short(&self) -> Option<&str> {
        if let Some(index) = self.combined.find('|')
        {
            Some(&self.combined[..index])
        }
        else
        {
            None
        }
    }

    fn combined(&self) -> &str {
        &self.combined
    }
}

fn main(){

}

#[cfg(test)]
mod tests
{
    use crate::MsiDirectoryName;

    #[test]
    fn test_parsing()
    {
        let str1 : &str = ".:Alpha";
        let dir1 : MsiDirectoryName = MsiDirectoryName::from(str1);
        let dir1_src = dir1.source().unwrap();
        let dir1_tgt = dir1.target();
        
        assert_eq!(dir1_src.long(), ".");
        assert_eq!(dir1_src.short().is_some(), false);
        assert_eq!(dir1_tgt.long(), "Alpha");
        assert_eq!(dir1_tgt.short().is_some(), false);

        let str2 : &str = ".:PROGRA~1|Program Files (x86)";
        let dir2 : MsiDirectoryName = MsiDirectoryName::from(str2);
        let dir2_src = dir2.source().unwrap();
        let dir2_tgt = dir2.target();
                
        assert_eq!(dir2_src.long(), ".");
        assert_eq!(dir2_src.short().is_some(), false);
        assert_eq!(dir2_tgt.long(), "Program Files (x86)");
        assert_eq!(dir2_tgt.short().unwrap(), "PROGRA~1");

        let str3 : &str = "SRCDIR|SourceDir:Alpha";
        let dir3 : MsiDirectoryName = MsiDirectoryName::from(str3);
        let dir3_src = dir3.source().unwrap();
        let dir3_tgt = dir3.target();
                
        assert_eq!(dir3_src.long(), "SourceDir");
        assert_eq!(dir3_src.short().unwrap(), "SRCDIR");
        assert_eq!(dir3_tgt.long(), "Alpha");
        assert_eq!(dir3_tgt.short().is_some(), false);

        let str4 : &str = "TARGETDIR";
        let dir4 : MsiDirectoryName = MsiDirectoryName::from(str4);
        let dir4_src = dir4.source();
        let dir4_tgt = dir4.target();
                
        assert_eq!(dir4_src.is_some(), false);  
        assert_eq!(dir4_tgt.long(), "TARGETDIR");
        assert_eq!(dir4_tgt.short().is_some(), false);        
    }
}