pub(crate) struct VorbisVector{
    comments : Vec<(String, Vec<String>)>
}

impl Default for VorbisVector {
    fn default() -> Self {
        Self { comments: Default::default() }
    }
}

impl VorbisVector {
    pub (crate) fn new() -> Self {
        Self {
            comments : vec![]
        }
    }

    pub (crate) fn iter(&self) -> impl Iterator<Item = (&String, String)>  {
        VorbisVectorIter {
            vorbis_vector: self,
            index: 0
        }
    }

    pub (crate) fn add(&mut self, key : &str, value: &str) {
        let key = key.to_uppercase();
        if let Some((_, ref mut values)) = self.comments.iter_mut().find(|(k, v)| *k == key && !v.contains(&value.to_string())){
            values.push(value.to_owned());
        }else {
            self.comments.push((key, vec![value.to_owned()] ))
        }
    }

    pub (crate) fn set(&mut self, key: &str, value: &str) {
        let key = key.to_uppercase();
        if let Some((_, ref mut values)) = self.comments.iter_mut().find(|(k,_)| *k == key ){
            values.clear();
            values.push(value.to_owned());
        }else {
            self.comments.push((key, vec![value.to_owned()] ))
        }
    }

    pub (crate) fn remove(&mut self, key : &str) {
        let key = key.to_uppercase();
        self.comments.retain(|(k,_)| k != &key)
    }

    pub (crate) fn _get_raw(&self, key : &str) -> Option<&Vec<String>> {
        self.comments.iter().find_map(|(k,v)| if k == key {Some(v)} else {None})
    }

    pub (crate) fn get(&self, key : &str) -> Option<String> {
        let key = key.to_uppercase();
        self.comments.iter()
        .find_map(|(k,v)| if *k == key { Some(v.join(",")) } else { None })
    }

    pub(crate) fn len(&self) -> usize {
        self.comments.len()
    }
}

pub (crate) struct VorbisVectorIter<'a> {
    vorbis_vector : &'a VorbisVector,
    index : usize
}


impl<'a> Iterator for VorbisVectorIter<'a> {
    type Item = (&'a String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vorbis_vector.len(){
            None
        }else {
            let (k,v) = self.vorbis_vector.comments.get(self.index).unwrap();
            self.index +=1;
            Some((k, v.join(",")))
        }
    }
}