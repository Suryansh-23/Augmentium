import { motion } from "framer-motion";
import React from "react";

const Animated = ({
    children,
    delay = 0,
    className,
}: {
    children: React.ReactNode;
    delay?: number;
    className?: string;
}) => {
    return (
        <motion.div
            initial={{ opacity: 0, transform: "translateY(25px)" }}
            whileInView={{ opacity: 1, transform: "translateY(0)" }}
            transition={{ duration: 1 }}
            viewport={{ once: true }}
            className={className}>
            {children}
        </motion.div>
    );
};

export default Animated;
