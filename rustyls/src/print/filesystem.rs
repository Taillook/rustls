use std::{path::PathBuf, fmt, os::unix::fs::PermissionsExt};

pub trait FileSystem {
    fn permission(&self) -> Permission;
}

#[derive(Copy, Clone)]
pub struct Permission {
    pub is_file:        bool,

    pub user_read:      bool,
    pub user_write:     bool,
    pub user_execute:   bool,

    pub group_read:     bool,
    pub group_write:    bool,
    pub group_execute:  bool,

    pub other_read:     bool,
    pub other_write:    bool,
    pub other_execute:  bool,

    pub sticky:         bool,
    pub setgid:         bool,
    pub setuid:         bool,
}

impl Permission {
    fn user_execute_bit(&self, is_file: bool) -> &str {
        match (self.user_execute, self.setuid, is_file) {
            (false, false, _)      => "-",
            (true,  false, false)  => "x",
            (true,  false, true)   => "x",
            (false, true,  _)      => "S",
            (true,  true,  false)  => "S",
            (true,  true,  true)   => "S",
        }
    }

    fn group_execute_bit(&self) -> &str {
        match (self.group_execute, self.setgid) {
            (false, false)  => "-",
            (true,  false)  => "x",
            (false, true)   => "S",
            (true,  true)   => "s",
        }
    }

    fn other_execute_bit(&self) -> &str {
        match (self.other_execute, self.sticky) {
            (false, false)  => "-",
            (true,  false)  => "x",
            (false, true)   => "T",
            (true,  true)   => "t",
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        fn bit (bit: bool, chr: &str) -> &str {
            if bit { chr } else { "-" }
        };
        write!(f, "{}{}{}{}{}{}{}{}{}", bit(self.user_read, "r"), bit(self.user_write, "w"), self.user_execute_bit(self.is_file), bit(self.group_read, "r"), bit(self.group_write, "w"), self.group_execute_bit(), bit(self.other_read, "r"), bit(self.other_write, "w"), self.other_execute_bit())
    }
}

impl FileSystem for PathBuf {
    fn permission(&self) -> Permission {
        let metadata = self.metadata().expect("metadata call failed");
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        let has_bit = |bit| mode & bit == bit;

        Permission {
            is_file:        self.is_file(),

            user_read:      has_bit(libc::S_IRUSR as u32),
            user_write:     has_bit(libc::S_IWUSR as u32),
            user_execute:   has_bit(libc::S_IXUSR as u32),

            group_read:     has_bit(libc::S_IRGRP as u32),
            group_write:    has_bit(libc::S_IWGRP as u32),
            group_execute:  has_bit(libc::S_IXGRP as u32),

            other_read:     has_bit(libc::S_IROTH as u32),
            other_write:    has_bit(libc::S_IWOTH as u32),
            other_execute:  has_bit(libc::S_IXOTH as u32),

            sticky:         has_bit(libc::S_ISVTX as u32),
            setgid:         has_bit(libc::S_ISGID as u32),
            setuid:         has_bit(libc::S_ISUID as u32),
        }
    }
}
