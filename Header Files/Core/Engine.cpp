#include "Engine.h"

//Additional include files
#include "Renderer.h"

#include "System.h"
#include "Game.h"
#include "Window.h"
#include "Graphics.h"

#include "GraphicsDeviceManager.h"

#ifndef _DELETEMACRO_H
	#include "deletemacros.h"
#endif
#ifndef _STRING_H
	#include "string.h"
#endif

EngineState Engine::m_EngineState = EngineState::Invalid;

Engine::Engine() {

	m_EngineState = EngineState::Constructing;
}


Engine::~Engine() {

	m_EngineState = EngineState::Destroying;
}

int Engine::RunLoop() {

	Context context;
	context.pRenderer = new Renderer();

	if (!this->Initialize())
		return 0;

	srand(GetTickCount());

	MSG msg = {};

	m_EngineState = EngineState::Runnig;

	while (msg.message != WM_QUIT && m_EngineState == EngineState::Runnig) {

		CheckResize();

		if (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {

			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}

		this->Update(context);
		this->Draw(context);
	}

	//Logger::Log("Ending program");
	//Logger::WriteLogFile();

	if (!this->ShutDown())
		return 0;

	return msg.wParam;
}

int Engine::Initialize() {

	m_EngineState = EngineState::Initializing;

	Game* game = CreateGame();

	if (!game)
		return false;

	//Add some systems
	if (!AddSystem(new Window(WindowData(640, 480))))
		return false;
	if (!AddSystem(new Graphics(GraphicsData(GetSystem<Window>(SystemType::Sys_Window)))))
		return false;

	//Initialize the system
	if (!m_mapSystems[SystemType::Sys_Window]->Initialize())
		return false;
	if (!m_mapSystems[SystemType::Sys_Graphics]->Initialize())
		return false;

	GRAPHICSDEVICEMANAGER->SetGraphics(GetSystem<Graphics>(SystemType::Sys_Graphics));

	return true;
}

int Engine::Draw(Context& context) {
	
	Graphics* graph = GetSystem<Graphics>(SystemType::Sys_Graphics);
	if (graph == nullptr)
		return false;

	graph->BeginDraw();

	//Draw the game
	RENDERER->SetColor(Color(1, 0, 0, 1));
	RENDERER->FillRect(100, 100, 300, 200);

	RENDERER->SetColor(Color(0, 1, 0, 1));
	RENDERER->FillRect(200, 200, 300, 200);


	graph->EndDraw();

	return true;
}
int Engine::Update(Context& context) {

	for (std::pair<SystemType, System*> pSys : m_mapSystems) {

		if (!pSys.second)
			continue;

		pSys.second->Update(context);
	}
	return true;
}
int Engine::ShutDown() {

	m_EngineState = EngineState::ShuttingDown;

	for (std::pair<SystemType, System*> psys : m_mapSystems) {

		//if (!psys.second->ShutDown()) {

			//Logger::Log("Faild to shutdown system " + psys->GetSystemType());
			//continue;
		//}

		SafeDelete(psys.second);
	}

	return true;
}

//Private Methods
void Engine::CheckResize() {

	//Find Window system
	//if the window has been found, check if it's valid
	//Get resize data from the window
	Window* wnd = GetSystem<Window>(SystemType::Sys_Window);
	if (wnd && wnd->GetResizeData().mustResize) {

		//If we need to resize 
		//find the graphics system in the system map 
		//If the graphics has been found, check if it's valid
		Graphics* graph = GetSystem<Graphics>(SystemType::Sys_Graphics);
		if (graph) {

			//Fire the resize method from the graphics class
			//Set the resize data  from the window back to false
			graph->OnResize(wnd->GetResizeData().newWidth, wnd->GetResizeData().newHeight);
			wnd->GetResizeData().mustResize = false;
		}
	}
}

int Engine::AddSystem(System* psys) {

	auto element = m_mapSystems.insert(std::make_pair(psys->GetType(), psys));
	if (element.second)
		return true;

	return false;
}

Game* Engine::CreateGame() {

	if (!AddSystem(new Game(GameData())))
		return nullptr;

	Game* game = GetSystem<Game>(SystemType::Sys_Game);

	if (!game)
		return nullptr;

	//if (!game->Initialize())
	//	return nullptr;

	return game;
}