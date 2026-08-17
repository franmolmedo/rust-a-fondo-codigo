use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Counted<F> {
    future: F,     // estructuralmente pinneado: es la máquina async
    polls: u32,    // no pinneado: un u32 puede moverse sin riesgo
}

impl<F> Counted<F> {
    fn project(self: Pin<&mut Self>) -> (Pin<&mut F>, &mut u32) {
        // SAFETY: `future` nunca se mueve fuera de `self` ni se
        // reemplaza: solo se re-pinnea. Entregar `polls` como &mut
        // no permite mover `future`, y `Counted` no implementa
        // `Unpin` a la carta ni un Drop que mueva campos.
        unsafe {
            let this = self.get_unchecked_mut();
            (Pin::new_unchecked(&mut this.future), &mut this.polls)
        }
    }
}

impl<F: Future> Future for Counted<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let (future, polls) = self.project();
        *polls += 1;
        future.poll(cx)
    }
}
