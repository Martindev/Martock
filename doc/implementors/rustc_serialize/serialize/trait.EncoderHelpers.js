(function() {var implementors = {};
implementors["sdl2_window"] = [];implementors["image"] = [];implementors["opengl_graphics"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
