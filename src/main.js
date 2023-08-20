const { invoke } = window.__TAURI__.tauri;

let todoList;

async function getTodos() {
  invoke('get_todos')
    .then((todos) => {
      todos.forEach((t) => {
        if (!document.getElementById(`todo-${t.id}`)) {
          const liEl = document.createElement('li');
          liEl.id = `todo-${t.id}`;
          liEl.innerText = t.text;

          todoList.appendChild(liEl);
        }
      });
    })
    .catch((e) => alert(e));
}

async function addTodo(e) {
  console.log(document.getElementById('todoText'));
  invoke('add_todo', {
    todo: { text: document.getElementById('todoText').value },
  })
    .then(() => getTodos())
    .catch((e) => alert(e));
}

window.addEventListener('DOMContentLoaded', () => {
  todoList = document.getElementById('todos-list');

  document.getElementById('add-todo').addEventListener('submit', (e) => {
    e.preventDefault();
    addTodo(e);
  });
  getTodos();
});
