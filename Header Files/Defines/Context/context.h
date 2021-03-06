#ifndef _CONTEXT_H
#define _CONTEXT_H

//Forward Declaration
class Window;
class Renderer;


#define WINDOW (context.pWnd)
#define RENDERER (context.pRenderer)

struct Context {

	float dTime;

	Window* pWnd;
	Renderer* pRenderer;
};

#endif
