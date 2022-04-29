#[cfg(test)]

mod task_tests {

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tasks::finite_task::FiniteTask;
    use crate::tasks::infinite_task::InfiniteTask;
    use crate::tasks::parallel_task::ParallelTask;
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

    #[test]
    fn parallel_tasks() {
        let a = Rc::new(RefCell::new(0));
        let a1 = a.clone();
        let b = Rc::new(RefCell::new(0));
        let b1 = b.clone();

        let t1 = InfiniteTask::new(move || {
            *a.borrow_mut() += 1;
        });

        let t2 = InfiniteTask::new(move || {
            *b.borrow_mut() += 1;
        });

        let mut task = ParallelTask::new(Box::new(t1), Box::new(t2));

        assert_eq!(*a1.borrow(), 0);
        assert_eq!(*b1.borrow(), 0);

        task.execute();
        assert_eq!(*a1.borrow(), 1);
        assert_eq!(*b1.borrow(), 1);

        task.execute();
        task.execute();
        task.execute();
        task.execute();
        assert_eq!(*a1.borrow(), 5);
        assert_eq!(*b1.borrow(), 5);

        assert_eq!(task.has_finished(), false);
    }
}