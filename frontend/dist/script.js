app.ports.deleteSchedule.subscribe((schedule) => {
  const confirmDeletion = confirm(
    `Really delete schedule ${schedule.hour}:${schedule.minute}?`
  );

  if (confirmDeletion) {
    app.ports.scheduleDeleteRecv.send(schedule);
  }
});
