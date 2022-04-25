#[cfg(test)]

mod task_tests {

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tasks::finite_task::FiniteTask;
    use crate::tasks::infinite_task::InfiniteTask;
    use crate::tasks::task::Task;

    #[test]
    fn infinite_task() {
        let c = Rc::new(RefCell::new(0));
        let c1 = c.clone();
        let mut t = InfiniteTask::new(|| {
            *c.borrow_mut() += 1;
        });

        t.execute();
        assert_eq!(*c1.borrow(), 1);

        t.execute();
        t.execute();
        t.execute();
        t.execute();
        assert_eq!(*c1.borrow(), 5);
    }

    #[test]
    fn finite_task() {
        let c = Rc::new(RefCell::new(0));
        let c1 = c.clone();

        let mut t = FiniteTask::new(|| {
            *c.borrow_mut() += 1;
        }, || {}, || {}, || {*c1.borrow_mut() > 5});

        t.execute();
        assert_eq!(t.has_finished(), false);
        t.execute();
        t.execute();
        t.execute();
        t.execute();
        assert_eq!(t.has_finished(), false); // c = 5, c is not > 5
        t.execute();
        assert_eq!(t.has_finished(), true);
    }
}