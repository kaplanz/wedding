const pluralize = (count, noun, suffix = "s") =>
  `${count} ${noun}${count !== 1 ? suffix : ""}`;

function countdown(deadline) {
    const now   = new Date();
    const total = deadline - now;
    return {
        total,
        days:    Math.floor( total / (1000 * 60 * 60 * 24)),
        hours:   Math.floor((total /  1000  / 60) % 60),
        minutes: Math.floor((total / (1000  * 60 *  60)) % 24),
        seconds: Math.floor((total /  1000) % 60),
    }
}

function update(elem, deadline) {
    // Calculate the countdown
    const dur = countdown(deadline);
    const days = Math.max(0, dur.days + 1);
    // Update the element
    elem.innerHTML = pluralize(days, "day");
}

// Mark the deadline
const deadline = new Date("2023-06-06 00:00:00");
const elem = document.getElementById("days");

// Set the initial countdown
update(elem, deadline);
