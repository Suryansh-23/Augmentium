import { useEffect } from "react";

const MeteorShower = () => {
    useEffect(() => {
        const createMeteor = () => {
            // create a meteor
            let meteor = document.createElement("div");
            meteor.setAttribute("class", "meteor");
            meteor.style.left =
                Math.round(Math.random() * window.innerWidth) + "px";
            meteor.style.top =
                Math.round(Math.random() * window.innerHeight) + "px";

            // append the meteor to a random meteor shower (direction)
            const showers = document.querySelectorAll(".shower");
            const random = Math.floor(Math.random() * showers.length);
            const shower = showers[random];
            shower.append(meteor);

            // remove the meteor after the animation duration
            setTimeout(() => {
                meteor.remove();
            }, 3500);
        };

        // Create meteors on interval every 1.5 seconds
        const intervalId = setInterval(createMeteor, 1500);

        // Clear interval on component unmount
        return () => clearInterval(intervalId);
    }, []);

    return (
        <div id="meteors">
            {/* rotations defined in base.css & tailwind.config.mjs */}
            <div className="shower ur"></div>
            <div className="shower dr"></div>
            <div className="shower dl"></div>
            <div className="shower ul"></div>
        </div>
    );
};

export default MeteorShower;
