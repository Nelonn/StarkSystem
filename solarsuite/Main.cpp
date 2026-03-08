
#include <cstdio>
#include <iostream>

extern "C" {
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>
}

auto main(int argc, char** argv) -> int {
  if (argc < 2) {
    printf("Usage: solarsuite [options]\n");
  }

  lua_State* l = luaL_newstate();
  if (!l) {
    std::cerr << "Failed to create lua state" << std::endl;
    return 1;
  }
  luaL_openlibs(l);

  if (luaL_dofile(l, "script.lua") != LUA_OK) {
    std::cerr << "Error: " << lua_tostring(l, -1) << std::endl;
    lua_pop(l, 1);
    return 1;
  }

  lua_getglobal(l, "multiply");
  lua_pushnumber(l, 5);
  lua_pushnumber(l, 3.2);
  if (lua_pcall(l, 2, 1, 0) != LUA_OK) {
    std::cerr << "Error: " << lua_tostring(l, -1) << std::endl;
    lua_pop(l, 1);
    return 1;
  }

  double result = lua_tonumber(l, -1);
  lua_pop(l, 1);

  std::cout << result << std::endl;

  lua_close(l);

  return 0;
}
