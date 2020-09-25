#ifndef polyhorn_springboard_h
#define polyhorn_springboard_h

typedef struct {
    void (*main)(void);
    void (*application_did_finish_launching)(void);
} PLYSpringboard;

extern PLYSpringboard poly_springboard;

#endif /* polyhorn_springboard_h */
