(function() {var implementors = {};
implementors["rand"] = [];implementors["sdl2"] = [];implementors["image"] = [];implementors["sdl2_window"] = [];implementors["opengl_graphics"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()