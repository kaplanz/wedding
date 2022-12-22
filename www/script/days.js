const pluralize = (count, noun, suffix = "s") =>
  `${count} ${noun}${count !== 1 ? suffix : ""}`;

function days(deadline) {
    const now   = new Date();
    const total = deadline - now;
    const duration = {
        total,
        days:    Math.floor( total / (1000 * 60 * 60 * 24)),
        hours:   Math.floor((total /  1000  / 60) % 60),
        minutes: Math.floor((total / (1000  * 60 *  60)) % 24),
        seconds: Math.floor((total /  1000) % 60),
    };

    const days = document.getElementById("days");
    days.innerHTML = pluralize(duration.days, "day");
}

// Mark the deadline
const deadline = new Date("2023-06-06 00:00:00");

// Set the initial tick
days(deadline);
