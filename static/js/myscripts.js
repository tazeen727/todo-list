function addTask() {
    const list = document.getElementById('tasklist');
    const size = list.childElementCount;
    const taskId = 'task' + (size + 1);

    const newtaskDesc = document.getElementById('newtask_desc');
    const description = newtaskDesc.value;
    newtaskDesc.value = '';

    appendTask(taskId, description, false);
}

function appendTask(taskId, description, done) {
    const list = document.getElementById('tasklist');
    const li = document.createElement('li');

    const label = document.createElement('label');
    label.setAttribute('for', taskId);
    label.textContent = description;

    const chkbox = document.createElement('input');
    chkbox.setAttribute('type', 'checkbox');
    chkbox.setAttribute('id', taskId);
    chkbox.addEventListener('change', function (event) {
        if (event.target.checked) {
            label.classList.add('done');
        } else {
            label.classList.remove('done');
        }
    });
    chkbox.checked = done;

    li.appendChild(chkbox);
    li.appendChild(label);
    list.appendChild(li);

    return chkbox;
}

function updateTasklist() {
    // Remove all children
    const tasklist = document.getElementById('tasklist');
    while (tasklist.firstChild) {
        tasklist.removeChild(tasklist.firstChild);
    }

    fetch('/tasks')
        .then((res) => {
            if (res.ok) {
                return res.json();
            } else {
                return new Error(`Request failed: ${res.status}`);
            }
        })
        .then((tasks) => {
            for (i in tasks) {
                const chkbox = appendTask(tasks[i].id, tasks[i].description, tasks[i].done);
                chkbox.dispatchEvent(new Event('change'));
            }
        })
        .catch((err) => console.error(err));
}



window.addEventListener('DOMContentLoaded', function () {
    const btn = document.getElementById('newtask_button');
    btn.addEventListener('click', addTask);

    const btn2 = document.getElementById('send_button');
    btn2.addEventListener('click', updateTasklist);
});
