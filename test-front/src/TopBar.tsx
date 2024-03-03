import {
  AppBar,
  Avatar,
  Box,
  Button,
  Divider,
  IconButton,
  ListItemIcon,
  Menu,
  MenuItem,
  Toolbar,
  Tooltip,
} from "@mui/material";
import { memo, useState, MouseEvent } from "react";
import { useUser } from "./hooks/useUser";

import { Logout } from "@mui/icons-material";
import { useLogout } from "./hooks/useLogout";

const TopBar = (): JSX.Element => {
  const { user, loading } = useUser();
  const { logout } = useLogout();

  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <Box mb={1}>
      <AppBar position="static">
        <Toolbar>
          <Box flexGrow="1">
            <Button href="/" color="inherit">
              test-app
            </Button>
          </Box>
          {user ? (
            <Box>
              <Box>
                <Tooltip title="アカウント設定">
                  <IconButton onClick={handleClick}>
                    <Avatar />
                  </IconButton>
                </Tooltip>
              </Box>
              <Menu
                anchorEl={anchorEl}
                open={open}
                onClose={handleClose}
                onClick={handleClose}
              >
                <Divider />
                <MenuItem onClick={logout}>
                  <ListItemIcon>
                    <Logout />
                  </ListItemIcon>
                  ログアウト
                </MenuItem>
              </Menu>
            </Box>
          ) : (
            !loading && (
              <Button href="/login" color="inherit">
                ログインはこちらから
              </Button>
            )
          )}
        </Toolbar>
      </AppBar>
    </Box>
  );
};

const NamedTopBar = memo(TopBar);
export default NamedTopBar;
