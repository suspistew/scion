use hecs::Entity;

/// `FocusManager` is the resource that will handle the cycle of focus with tab
/// coupled to the focus systems
#[derive(Default)]
pub struct FocusManager{
    current_focus_index: Option<usize>,
    current_focus_entity: Option<Entity>
}

impl FocusManager{
    pub(crate) fn current_focus_index(&self) -> Option<usize>{
        if self.current_focus_index.is_none(){
            return None;
        }
        return Some(self.current_focus_index.unwrap())
    }

    pub(crate) fn current_focus_entity(&self) -> Option<Entity>{
        if self.current_focus_entity.is_none(){
            return None;
        }
        return Some(self.current_focus_entity.unwrap())
    }

    pub(crate) fn change_focus(&mut self, entity: Entity, rank: usize){
        self.current_focus_index = Some(rank);
        self.current_focus_entity = Some(entity);
    }

    pub(crate) fn reset_focus(&mut self){
        self.current_focus_index = None;
    }
}