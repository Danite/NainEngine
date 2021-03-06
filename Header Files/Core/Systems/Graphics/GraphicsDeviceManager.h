#ifndef _GRAPHICSDEVICEMANAGER_H
#define _GRAPHICSDEVICEMANAGER_H

//Forward declaration
class Graphics;


#define GRAPHICSDEVICEMANAGER (GraphicsDeviceManager::GetInstance())

class GraphicsDeviceManager
{
	friend class Engine;
public:

	~GraphicsDeviceManager();

	static GraphicsDeviceManager* GetInstance();

	Graphics* GetGraphics() { return m_pGraphics; }

private:

	GraphicsDeviceManager();

	static GraphicsDeviceManager* m_pInstance;

	void SetGraphics(Graphics* graphics) { m_pGraphics = graphics; }

	Graphics* m_pGraphics;
};

#endif