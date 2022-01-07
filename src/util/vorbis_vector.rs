pub(crate) struct VorbisVector{
    comments : Vec<(String, Vec<String>)>
}

impl VorbisVector {
    pub (crate) fn new() -> Self {
        Self {
            comments : vec![]
        }
    }

    pub (crate) fn add(&mut self, key : &str, value: &str) {
        if let Some((_, ref mut values)) = self.comments.iter_mut().find(|(k,_)| k == key ){
            values.push(value.to_owned());
        }else {
            let key = key.to_uppercase();
            self.comments.push((key, vec![value.to_owned()] ))
        }
    }

    pub (crate) fn set(&mut self, key: &str, value: &str) {
        if let Some((_, ref mut values)) = self.comments.iter_mut().find(|(k,_)| k == key ){
            values.clear();
            values.push(value.to_owned());
        }else {
            let key = key.to_uppercase();
            self.comments.push((key, vec![value.to_owned()] ))
        }
    }

    pub (crate) fn remove(&mut self, key : &str) {
        self.comments.retain(|(k,_)| k != key)
    }

    pub (crate) fn get_raw(&mut self, key : &str) -> Option<&Vec<String>> {
        self.comments.iter().find_map(|(k,v)| if k == key {Some(v)} else {None})
    }

    pub (crate) fn get(&mut self, key : &str) -> Option<String> {
        self.comments.iter()
        .find_map(|(k,v)| if k == key { Some(v.join(", ")) } else { None })

    }
}